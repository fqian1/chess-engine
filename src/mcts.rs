use core::fmt;
use log::info;
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::io::Read;

use burn::prelude::Backend;

use crate::{
    ChessGame, ChessMove, ChessPosition, ChessSquare, ChessTransformer, Color, NetworkInputs, NetworkLabels, PieceType, TrainingConfig,
    TrainingSample, XorShift64, chess_game::Outcome, model_make_outputs,
};

#[derive(Default, Debug, Copy, Clone)]
pub struct NodeData {
    chess_position_idx: usize,                // assigned on creation
    child_edge_range: Option<(usize, usize)>, // assigned on expansion
    parent_edge_idx: Option<usize>,           // assigned on creation (None if root), but unused. remove?
    value: Option<[f32; 3]>,                  // assigned on expansion
    is_terminal: bool,                        // assigned on traversal
    visits: usize,                            // updated on traversal
}

impl NodeData {
    pub fn new(chess_position_idx: usize, parent_edge_idx: usize) -> Self {
        Self { chess_position_idx, child_edge_range: None, parent_edge_idx: Some(parent_edge_idx), value: None, is_terminal: false, visits: 0 }
    }
}

// --------------------

#[derive(Debug, Clone)]
pub enum MctsNode {
    PieceSelect { data: NodeData },
    PieceMove { data: NodeData, from_sq: ChessSquare },
}

impl fmt::Display for MctsNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MctsNode::PieceSelect { data } => write!(
                f,
                "Select: position_idx: {}, edges: {:?}, value: {:?}, terminal: {}, visits: {}",
                data.chess_position_idx, data.child_edge_range, data.value, data.is_terminal, data.visits
            ),
            MctsNode::PieceMove { data, from_sq } => write!(
                f,
                "Select: from: {} position_idx: {}, edges: {:?}, value: {:?}, terminal: {}, visits: {}",
                from_sq.to_name(),
                data.chess_position_idx,
                data.child_edge_range,
                data.value,
                data.is_terminal,
                data.visits
            ),
        }
    }
}

impl Default for MctsNode {
    fn default() -> Self {
        MctsNode::PieceSelect { data: NodeData::default() }
    }
}

impl MctsNode {
    fn get_data_mut(&mut self) -> &mut NodeData {
        match self {
            Self::PieceSelect { data } => data,
            Self::PieceMove { data, .. } => data,
        }
    }

    fn get_data(&self) -> &NodeData {
        match self {
            Self::PieceSelect { data } => data,
            Self::PieceMove { data, .. } => data,
        }
    }
}

// --------------------

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MctsEdge {
    square: ChessSquare,
    confidence: f32, // the policy prob
    visits: u32,
    total_value: [f32; 3],         // cumulative value of leaf nodes
    mean_value: [f32; 3],          // total val / visits
    child_node_idx: Option<usize>, // None if not explored
    parent_node_idx: usize,
    promotion_piece: Option<PieceType>,
}

impl fmt::Display for MctsEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "sq: {}, conf: {}, visits: {}, acc_val: {:?}, mea_val: {:?}",
            self.square, self.confidence, self.visits, self.total_value, self.mean_value
        )
    }
}

impl MctsEdge {
    pub fn new(square: ChessSquare, confidence: f32, parent_node_idx: usize) -> Self {
        Self {
            square,
            confidence,
            visits: 0,
            total_value: [0.0; 3],
            mean_value: [0.0; 3],
            child_node_idx: None,
            parent_node_idx,
            promotion_piece: None,
        }
    }

    pub fn with_prom(mut self, piece: PieceType) -> Self {
        self.promotion_piece = Some(piece);
        self
    }

    pub fn to_value(&self) -> f32 {
        let (w, d, l) = (self.mean_value[0], self.mean_value[1], self.mean_value[2]);
        (w - l) / (w + d + l)
    }
}

// --------------------

#[derive(Debug, Default, Copy, Clone)]
pub struct MctsConfig {
    pub num_simulations: usize,
    pub c_puct: f32,
    pub temperature: f32,
    pub legal: bool,
}

pub struct Mcts {
    pub config: MctsConfig,
    pub node_arena: Vec<MctsNode>,
    pub edge_arena: Vec<MctsEdge>,
    pub position_arena: Vec<ChessPosition>,
    pub path: Vec<usize>, // idx of edges
    pub rng: XorShift64,
    pub root: usize,           // node idx
    pub past_hashes: Vec<u64>, // when make move
}

impl Mcts {
    pub fn from_game(game: &ChessGame, size: usize, config: MctsConfig) -> Self {
        let node = MctsNode::PieceSelect {
            data: NodeData { chess_position_idx: 0, child_edge_range: None, parent_edge_idx: None, value: None, is_terminal: false, visits: 0 },
        };
        let mut node_arena = Vec::with_capacity(size * 2);
        node_arena.push(node);

        let mut position_arena = Vec::with_capacity(size);
        position_arena.push(game.position.clone());

        let mut buffer = Vec::new();
        let _ = std::fs::File::open("/dev/urandom").unwrap().take(8).read_to_end(&mut buffer);
        let rng = XorShift64::new(u64::from_le_bytes(*buffer.as_array().expect("something wrong with rng")));

        let past_hashes: Vec<_> = game.game_history.iter().map(|game| game.zobrist_hash).collect();

        Self {
            config,
            node_arena: node_arena,
            edge_arena: Vec::with_capacity(size * 16),
            position_arena: position_arena,
            rng: rng,
            path: Vec::new(),
            root: 0,
            past_hashes,
        }
    }

    pub fn select_best_edge(&mut self, node_idx: usize) -> Option<usize> {
        let node = &self.node_arena[node_idx];
        let position = &self.position_arena[node.get_data().chess_position_idx];

        let (start, end) = node.get_data().child_edge_range?;
        let visit_count = node.get_data().visits;

        let mut calc_puct = |edge_idx: usize| -> f32 {
            let edge = &self.edge_arena[edge_idx];
            let (w, d, l) = (edge.mean_value[0], edge.mean_value[1], edge.mean_value[2]);

            // contempt
            let mut exploitation = w - l - 0.05 * d;
            if position.side_to_move == Color::Black {
                exploitation *= -1.0;
            }

            let mut prior = edge.confidence;
            if node_idx == self.root {
                let pos_idx = &self.node_arena[self.root].get_data().chess_position_idx;
                // scale noise by pieces on board
                let noise_weight = 0.5 - (self.position_arena[*pos_idx].chessboard.all_pieces.count() as f32 / 40.0);

                let noise = self.rng.next_f32();

                prior = ((1.0 - noise_weight) * prior) + (prior * noise);
            }

            let exploration = prior * self.config.c_puct * ((visit_count as f32).sqrt() + 1e-8) / (1 + edge.visits) as f32;
            // info!("node visit count: {}, edge visits: {}", visit_count, edge.visits);
            exploitation + exploration
        };

        let idx = (start..end).max_by(|&x, &y| {
            let a = calc_puct(x);
            let b = calc_puct(y);
            a.total_cmp(&b)
        });

        // (start..end).for_each(|x| {
        //     let conf = self.edge_arena[x].confidence;
        //     info!("edge_idx: {}, confidence: {}, puct: {}", x, conf, calc_puct(x));
        // });
        idx
    }

    pub fn get_network_input(&self, node_idx: usize) -> NetworkInputs {
        let node = &self.node_arena[node_idx];
        match node {
            MctsNode::PieceSelect { data } => {
                let position = &self.position_arena[data.chess_position_idx];
                NetworkInputs::from_position(&position, None)
            }
            MctsNode::PieceMove { data, from_sq } => {
                let position = &self.position_arena[data.chess_position_idx];
                NetworkInputs::from_position(&position, Some(&from_sq))
            }
        }
    }

    pub fn backprop(&mut self, value: [f32; 3]) {
        self.path.iter().for_each(|&idx| {
            let edge = &mut self.edge_arena[idx];
            edge.total_value[0] += value[0];
            edge.total_value[1] += value[1];
            edge.total_value[2] += value[2];
            edge.visits += 1;
            edge.mean_value[0] = edge.total_value[0] / edge.visits as f32;
            edge.mean_value[1] = edge.total_value[1] / edge.visits as f32;
            edge.mean_value[2] = edge.total_value[2] / edge.visits as f32;
            self.node_arena[edge.parent_node_idx].get_data_mut().visits += 1;
        });
    }

    fn add_leaf(&mut self, edge_idx: usize) -> Option<usize> {
        let mut path_hashes = Vec::new();
        let root_pos_idx = self.node_arena[self.root].get_data().chess_position_idx;
        path_hashes.push(self.position_arena[root_pos_idx].zobrist_hash);
        self.path.iter().for_each(|&idx| {
            let edge = &self.edge_arena[idx];
            let Some(node_idx) = edge.child_node_idx else {
                return;
            };
            let node = &self.node_arena[node_idx];
            if matches!(node, MctsNode::PieceSelect { data: _ }) {
                let pos = &self.position_arena[node.get_data().chess_position_idx];
                path_hashes.push(pos.zobrist_hash);
            }
        });

        let edge = &mut self.edge_arena[edge_idx];
        if edge.child_node_idx.is_some() {
            return None;
        }

        let parent_node = &self.node_arena[edge.parent_node_idx];
        match parent_node {
            MctsNode::PieceSelect { data } => {
                let new_node = MctsNode::PieceMove { data: NodeData::new(data.chess_position_idx, edge_idx), from_sq: edge.square };
                self.node_arena.push(new_node);
                let node_idx = self.node_arena.len() - 1;
                edge.child_node_idx = Some(node_idx);
                return Some(node_idx);
            }
            MctsNode::PieceMove { from_sq, .. } => {
                let mov = ChessMove::new(*from_sq, edge.square, edge.promotion_piece);

                let mut position = self.position_arena[parent_node.get_data().chess_position_idx].clone();
                position.make_move(&mov);

                let idx = (0..self.position_arena.len()).into_par_iter().find_any(|e| &self.position_arena[*e].zobrist_hash == &position.zobrist_hash);

                let repeats = self.past_hashes.iter().filter(|&hash| hash == &position.zobrist_hash).count()
                    + path_hashes.iter().filter(|&hash| hash == &position.zobrist_hash).count();

                // info!("add_leaf: position ({}) repeated {} times", self.position_arena.len(), repeats);
                let outcome = if repeats >= 2 {
                    Outcome::Finished(None)
                } else {
                    position.check_game_state(self.config.legal)
                };

                let idx = if let Some(idx) = idx {
                    idx
                } else {
                    self.position_arena.push(position);
                    self.position_arena.len() - 1
                };

                let mut new_node = MctsNode::PieceSelect { data: NodeData::new(idx, edge_idx) };
                if matches!(outcome, Outcome::Finished(_)) {
                    new_node.get_data_mut().is_terminal = true;
                    new_node.get_data_mut().value = Some(outcome.to_f32().unwrap());
                }
                self.node_arena.push(new_node);

                let node_idx = self.node_arena.len() - 1;
                edge.child_node_idx = Some(node_idx);

                return Some(node_idx);
            }
        }
    }

    pub fn node_to_expand(&self) -> Option<usize> {
        let path = &self.path;
        if let Some(idx) = path.last() {
            return self.edge_arena[*idx].child_node_idx;
        }
        Some(self.root)
    }

    pub fn get_position(&self, node_idx: usize) -> ChessPosition {
        self.position_arena[self.node_arena[node_idx].get_data().chess_position_idx].clone()
    }

    pub fn make_targets(&mut self) -> Option<TrainingSample> {
        let node = &self.node_arena[self.root];
        let position = &self.position_arena[node.get_data().chess_position_idx];

        let inputs = match node {
            MctsNode::PieceSelect { .. } => NetworkInputs::from_position(position, None),
            MctsNode::PieceMove { from_sq, .. } => NetworkInputs::from_position(position, Some(from_sq)),
        };

        if node.get_data().child_edge_range.is_none() {
            return None;
        }

        let (start, end) = node.get_data().child_edge_range.unwrap();
        let total_visits: u32 = self.edge_arena[start..end].iter().map(|e| e.visits).sum();
        let mut target_policy = [0.0; 64];
        self.edge_arena[start..end].iter().for_each(|e| target_policy[e.square.0 as usize] += e.visits as f32 / total_visits as f32);

        let best_edge_idx = (start..end).max_by(|a, b| self.edge_arena[*a].visits.cmp(&self.edge_arena[*b].visits)).unwrap();
        let best_edge = &self.edge_arena[best_edge_idx];
        let targets = NetworkLabels { policy: target_policy, value: best_edge.mean_value };
        Some(TrainingSample { inputs, targets })
    }

    pub fn traverse_get_terminal(&mut self) -> bool {
        let mut current_node_idx = self.root;
        self.path.clear();

        loop {
            let Some(child_edge_idx) = self.select_best_edge(current_node_idx) else {
                // current node not expanded or is terminal
                let node = &self.node_arena[current_node_idx];
                if node.get_data().is_terminal {
                    let value = node.get_data().value.expect("Terminal node missing value");
                    self.backprop(value);
                    return true;
                }
                break;
            };

            self.path.push(child_edge_idx);

            let next_edge = &self.edge_arena[child_edge_idx];

            // if edge has child, move to it, else make it
            if let Some(next_node_idx) = next_edge.child_node_idx {
                current_node_idx = next_node_idx;
            } else {
                _ = self.add_leaf(child_edge_idx).expect("Node already expanded");
                break;
            }
        }
        return false;
    }

    pub fn get_mask(&self, node_idx: usize) -> [bool; 64] {
        let node = &self.node_arena[node_idx];

        match node {
            MctsNode::PieceSelect { data } => return self.position_arena[data.chess_position_idx].make_mask(self.config.legal, None),
            MctsNode::PieceMove { data, from_sq } => {
                return self.position_arena[data.chess_position_idx].make_mask(self.config.legal, Some(*from_sq));
            }
        }
    }

    pub fn get_move_to_play(&mut self) -> Option<ChessMove> {
        let root_node = &self.node_arena[self.root];
        if root_node.get_data().is_terminal {
            info!("root at node idx: {} is terminal", self.root);
            return None;
        }
        let (start, end) = root_node.get_data().child_edge_range.unwrap();
        let edges = &self.edge_arena[start..end];

        let ply_count = self.past_hashes.len();
        let piece_count = self.position_arena[self.node_arena[self.root].get_data().chess_position_idx].chessboard.all_pieces.count();
        let temperature = self.config.temperature * ((1.0 / (ply_count + 1) as f32 + (piece_count - 4) as f32 / 28.0) / 2.0);

        let selected_edge = if self.config.temperature > 0.0 {
            let inv_temp = 1.0 / temperature;
            let weights: Vec<f32> = edges.iter().map(|e| (e.visits as f32).powf(inv_temp)).collect();
            let total_weight: f32 = weights.iter().sum();

            let mut choice = self.rng.next_f32() * total_weight;

            let mut picked = &edges[0];

            for (edge, weight) in edges.iter().zip(weights.iter()) {
                choice -= weight;
                if choice <= 0.0 {
                    picked = edge;
                    break;
                }
            }
            if picked.child_node_idx.is_none() {
                picked = edges.iter().find(|e| e.child_node_idx.is_some()).expect("root node not expanded");
            }
            picked
        } else {
            edges.iter().max_by(|&a, &b| a.visits.cmp(&b.visits)).unwrap()
        };

        self.root = selected_edge.child_node_idx.expect("best edge doesnt have child");
        match root_node {
            MctsNode::PieceMove { from_sq, .. } => {
                if let Some(piece_type) = selected_edge.promotion_piece {
                    let hash = self.position_arena[self.node_arena[self.root].get_data().chess_position_idx].zobrist_hash;
                    self.past_hashes.push(hash);
                    return Some(ChessMove::new(*from_sq, selected_edge.square, Some(piece_type)));
                }
                let hash = self.position_arena[self.node_arena[self.root].get_data().chess_position_idx].zobrist_hash;
                self.past_hashes.push(hash);
                return Some(ChessMove::new(*from_sq, selected_edge.square, None));
            }
            _ => {
                return None;
            }
        }
    }
}

pub fn expand_batch<B: Backend>(mctss: &mut [Mcts], model: ChessTransformer<B>, config: &TrainingConfig, device: &B::Device) -> (u32, f64) {
    let mut inputs: Vec<NetworkInputs> = Vec::with_capacity(config.batch_size);

    let mut unique: u32 = 0;

    let masks: Vec<_> = mctss
        .iter()
        .map(|game| {
            let node_idx = game.node_to_expand().expect("path is none");
            inputs.push(game.get_network_input(node_idx));
            let node = &game.node_arena[node_idx];

            if node.get_data().is_terminal {
                return [false; 64];
            }
            unique += 1;

            let position_idx = node.get_data().chess_position_idx;
            let position = &game.position_arena[position_idx];
            match node {
                MctsNode::PieceSelect { .. } => {
                    return position.make_mask(config.legal, None);
                }
                MctsNode::PieceMove { from_sq, .. } => {
                    return position.make_mask(config.legal, Some(*from_sq));
                }
            }
        })
        .collect();

    // generate outputs
    let mut mask_in: Vec<bool> = vec![false; config.batch_size * 64];
    mask_in.par_chunks_mut(64).zip(masks.par_iter()).for_each(|(dest_chunk, src_mask)| {
        dest_chunk.copy_from_slice(src_mask);
    });

    let outputs: Vec<NetworkLabels> = model_make_outputs(model.clone(), &inputs, config, if config.masked { Some(mask_in) } else { None }, device);

    let illegal_rate: f64 = mctss
        .par_iter_mut()
        .zip(outputs.into_par_iter())
        .zip(masks.into_par_iter())
        .map(|((game, output), mask)| {
            let node_idx = game.node_to_expand().expect("path is None");
            let position = &game.get_position(node_idx);
            let start = game.edge_arena.len();
            let (policy, value) = (output.as_squares(), output.value);
            let node_to_expand = &mut game.node_arena[node_idx];
            // info!("{}\nterminal: {}", position, node_to_expand.get_data().is_terminal);
            if node_to_expand.get_data().is_terminal {
                // info!("expand_batch: terminal node, skipping expand");
                return 0.0;
            }
            assert!(node_to_expand.get_data().child_edge_range.is_none());

            let rate: f64 = mask.iter().zip(policy.iter()).map(|(legal, policy)| if !legal { policy.1 as f64 } else { 0.0 }).sum();

            // info!("\n{}", output);

            // todo! par iter this
            mask.iter().zip(policy.iter()).for_each(|(legal, policy)| {
                let (sq, score) = (policy.0, policy.1);

                if *legal {
                    match node_to_expand {
                        MctsNode::PieceMove { data: _, from_sq } => {
                            let mov = ChessMove::new(*from_sq, sq, None);
                            if let Some(moves) = position.expand_if_prom(mov) {
                                for mov in moves {
                                    let edge = MctsEdge::new(sq, score / 4.0, node_idx).with_prom(mov.promotion.unwrap());
                                    // info!("adding edge: {}", edge);
                                    game.edge_arena.push(edge);
                                }
                            } else {
                                let edge = MctsEdge::new(sq, score, node_idx);
                                // info!("adding edge: {}", edge);
                                game.edge_arena.push(edge);
                            }
                        }
                        MctsNode::PieceSelect { data: _ } => {
                            let edge = MctsEdge::new(sq, score, node_idx);
                            // info!("adding edge: {}", edge);
                            game.edge_arena.push(edge);
                        }
                    }
                }
            });
            let end = game.edge_arena.len();
            assert!(end - start != 0);

            // info!("path: {:?}", game.path.iter().map(|&x| game.edge_arena[x].clone()).collect::<Vec<MctsEdge>>());

            // update node
            node_to_expand.get_data_mut().child_edge_range = Some((start, end));
            node_to_expand.get_data_mut().value = Some(value);
            game.backprop(value);
            game.path = Vec::new();
            rate
        })
        .sum();
    // info!("{}", illegal_rate);
    (unique, illegal_rate / config.batch_size as f64)
}

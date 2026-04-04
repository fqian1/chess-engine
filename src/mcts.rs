use core::fmt;
use std::io::Read;

use burn::prelude::Backend;
use log::{debug, info, trace};

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
    pub path: Option<Vec<usize>>, // idx of edges
    pub root: usize,
}

impl Mcts {
    pub fn from_game(root: &ChessGame, size: usize, config: MctsConfig) -> Self {
        let node = MctsNode::PieceSelect {
            data: NodeData { chess_position_idx: 0, child_edge_range: None, parent_edge_idx: None, value: None, is_terminal: false, visits: 0 },
        };
        let mut node_arena = Vec::with_capacity(size * 2);
        node_arena.push(node);

        let mut position_arena = Vec::with_capacity(size);
        position_arena.push(root.position.clone());

        Self { config, node_arena: node_arena, edge_arena: Vec::with_capacity(size * 16), position_arena: position_arena, path: None, root: 0 }
    }

    fn get_child_idx(&self, edge_idx: usize) -> Option<usize> {
        self.edge_arena[edge_idx].child_node_idx
    }

    pub fn select_best_edge(&self, node_idx: usize) -> Option<usize> {
        let node = &self.node_arena[node_idx];
        let position = &self.position_arena[node.get_data().chess_position_idx];

        let (start, end) = node.get_data().child_edge_range?;
        let visit_count = node.get_data().visits;

        let calc_puct = |edge_idx: usize| -> f32 {
            let edge = &self.edge_arena[edge_idx];
            let (w, _d, l) = (edge.mean_value[0], edge.mean_value[1], edge.mean_value[2]);

            let mut exploitation = w - l;
            if position.side_to_move == Color::Black {
                exploitation *= -1.0;
            }

            let mut prior = edge.confidence;
            if node_idx == self.root {
                // TODO: u know what to do.
                let mut buffer = Vec::new();
                let _ = std::fs::File::open("/dev/random").unwrap().take(1).read_to_end(&mut buffer);
                let mut rng = XorShift64::new(buffer[0] as u64);
                let noise_weight = 0.25;
                let noise = rng.next() as f32;
                prior = ((1.0 - noise_weight) * prior) + (prior * noise);
            }

            let exploration = prior * self.config.c_puct * (visit_count as f32).sqrt() / (1 + edge.visits) as f32;
            exploitation + exploration
        };

        let idx = (start..end).max_by(|&x, &y| {
            let a = calc_puct(x);
            let b = calc_puct(y);
            a.total_cmp(&b)
        });
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
        if let Some(path) = &mut self.path {
            path.iter().for_each(|&idx| {
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
    }

    fn add_leaf(&mut self, edge_idx: usize) -> Option<usize> {
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
                let mut position = self.position_arena[parent_node.get_data().chess_position_idx].clone();
                let mov = ChessMove::new(*from_sq, edge.square, edge.promotion_piece);

                position.make_move(&mov);
                info!("making move: {}", &mov.to_uci());
                let outcome = position.check_game_state(self.config.legal);

                self.position_arena.push(position);

                let mut new_node = MctsNode::PieceSelect { data: NodeData::new(self.position_arena.len() - 1, edge_idx) };
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
        let path = self.path.as_ref()?;
        if let Some(idx) = path.last() {
            return self.edge_arena[*idx].child_node_idx;
        }
        Some(0)
    }

    pub fn get_position(&self, node_idx: usize) -> ChessPosition {
        self.position_arena[self.node_arena[node_idx].get_data().chess_position_idx].clone()
    }

    pub fn make_targets(&mut self) -> TrainingSample {
        let node = &self.node_arena[self.root];
        let position = &self.position_arena[node.get_data().chess_position_idx];

        let inputs = match node {
            MctsNode::PieceSelect { .. } => NetworkInputs::from_position(position, None),
            MctsNode::PieceMove { from_sq, .. } => NetworkInputs::from_position(position, Some(from_sq)),
        };

        let (start, end) = node.get_data().child_edge_range.unwrap();
        let total_visits: u32 = self.edge_arena[start..end].iter().map(|e| e.visits).sum();
        let mut target_policy = [0.0; 64];
        self.edge_arena[start..end].iter().for_each(|e| target_policy[e.square.0 as usize] += e.visits as f32 / total_visits as f32);

        let best_edge_idx = (start..end).max_by(|a, b| self.edge_arena[*a].visits.cmp(&self.edge_arena[*b].visits)).unwrap();
        let best_edge = &self.edge_arena[best_edge_idx];
        let targets = NetworkLabels { policy: target_policy, value: best_edge.mean_value };
        TrainingSample { inputs, targets }
    }

    pub fn traverse(&mut self) {
        let mut current_node_idx = self.root;
        self.path.get_or_insert_with(Vec::new).clear();

        loop {
            let Some(child_edge_idx) = self.select_best_edge(current_node_idx) else {
                // current node not expanded or is terminal
                let node = &self.node_arena[current_node_idx];
                if node.get_data().is_terminal {
                    let value = node.get_data().value.expect("Terminal node missing value");
                    self.backprop(value);
                    self.path = None;
                    break;
                }
                break;
            };

            let path = self.path.as_mut().unwrap();
            path.push(child_edge_idx);

            let next_edge = &self.edge_arena[child_edge_idx];

            // if edge has child, move to it, else make it
            if let Some(next_node_idx) = next_edge.child_node_idx {
                current_node_idx = next_node_idx;
            } else {
                _ = self.add_leaf(child_edge_idx).expect("Node already expanded");
                break;
            }
        }
    }

    pub fn get_mask(&self, node_idx: usize) -> [bool; 64] {
        let node = &self.node_arena[node_idx];
        if let Some(edge_idx) = node.get_data().parent_edge_idx {
            match node {
                MctsNode::PieceSelect { data } => self.position_arena[node.get_data().chess_position_idx].make_mask(self.config.legal, None),
                MctsNode::PieceMove { data, from_sq } => {
                    self.position_arena[node.get_data().chess_position_idx].make_mask(self.config.legal, Some(*from_sq))
                }
            }
        } else {
            self.position_arena[0].make_mask(self.config.legal, None)
        }
    }

    pub fn get_move(&mut self) -> Option<ChessMove> {
        info!("creating move from: {}", self.root);
        let edge_idx = self.select_best_edge(self.root).expect("root node not expanded");
        let edge = &self.edge_arena[edge_idx];

        self.root = edge.child_node_idx.expect("best edge doesnt have child");
        let node = &self.node_arena[edge.parent_node_idx];
        match node {
            MctsNode::PieceMove { from_sq, .. } => {
                if let Some(piece_type) = edge.promotion_piece {
                    return Some(ChessMove::new(*from_sq, edge.square, Some(piece_type)));
                }
                return Some(ChessMove::new(*from_sq, edge.square, None));
            }
            _ => {
                return None;
            }
        }
    }
}

pub fn expand_batch<B: Backend>(mctss: &mut [Mcts], model: ChessTransformer<B>, config: &TrainingConfig, device: &B::Device) {
    let mut masks: Vec<[bool; 64]> = Vec::with_capacity(config.batch_size);
    let mut inputs: Vec<NetworkInputs> = Vec::with_capacity(config.batch_size);

    let print_mask = |mask: &[bool; 64]| -> String {
        let mut ascii = String::new();
        for i in (0..8).rev() {
            for j in 0..8 {
                ascii.push((mask[i * 8 + j] as u8 + b'0') as char);
                ascii.push(' ');
            }
            ascii.push('\n');
        }
        ascii
    };

    mctss.iter().for_each(|game| {
        let node_idx = game.node_to_expand().expect("path is none");

        inputs.push(game.get_network_input(node_idx));

        let node = &game.node_arena[node_idx];
        let position_idx = node.get_data().chess_position_idx;
        let position = &game.position_arena[position_idx];
        match node {
            MctsNode::PieceSelect { .. } => {
                let mask = position.make_mask(config.legal, None);
                info!("{:?}", &node);
                info!("\n{}\n{}", position, print_mask(&mask));
                assert!(mask.iter().any(|&e| e == true) == true);
                masks.push(mask);
            }
            MctsNode::PieceMove { from_sq, .. } => {
                let mask = position.make_mask(config.legal, Some(*from_sq));
                info!("{}", &node);
                info!("\n{}\n{}", position, print_mask(&mask));
                assert!(mask.iter().any(|&e| e == true) == true);
                masks.push(mask);
            }
        }
    });

    // generate outputs
    let mut mask_in: Vec<bool> = vec![false; config.batch_size * 64];
    masks.iter().enumerate().for_each(|(i, mask)| mask_in[(i * 64)..(i * 64 + 64)].copy_from_slice(mask));

    let outputs: Vec<NetworkLabels> = model_make_outputs(model.clone(), &inputs, config, if config.masked { Some(mask_in) } else { None }, device);

    mctss.iter_mut().zip(outputs.into_iter()).zip(masks.into_iter()).for_each(|((game, output), mask)| {
        let node_idx = game.node_to_expand().expect("path is None");
        let position = &game.get_position(node_idx);
        let start = game.edge_arena.len();
        let (policy, value) = (output.as_squares(), output.value);
        let node_to_expand = &mut game.node_arena[node_idx];

        mask.iter().zip(policy.iter()).for_each(|(mask, output)| {
            let (sq, score) = (output.0, output.1);

            if *mask {
                match node_to_expand {
                    MctsNode::PieceMove { data, from_sq } => {
                        let mov = ChessMove::new(*from_sq, sq, None);
                        info!("expand: adding edge from: from_sq {}", from_sq.to_name());
                        if let Some(moves) = position.expand_if_prom(mov) {
                            for mov in moves {
                                let edge = MctsEdge::new(sq, score / 4.0, node_idx).with_prom(mov.promotion.unwrap());
                                game.edge_arena.push(edge);
                            }
                        } else {
                            let edge = MctsEdge::new(sq, score, node_idx);
                            game.edge_arena.push(edge);
                        }
                    }
                    MctsNode::PieceSelect { data } => {
                        info!("expand: adding edge for sq: {}", sq.to_name());
                        let edge = MctsEdge::new(sq, score, node_idx);
                        game.edge_arena.push(edge);
                    }
                }
            }
        });
        let end = game.edge_arena.len();
        info!("added {} edges", end - start);

        // update node
        node_to_expand.get_data_mut().child_edge_range = Some((start, end));
        node_to_expand.get_data_mut().value = Some(value);
        game.backprop(value);
        game.path = None;
    });
}

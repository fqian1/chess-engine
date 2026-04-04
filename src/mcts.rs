use burn::prelude::Backend;
use log::{debug, error, info, trace, warn};

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
    pub path: Vec<usize>, // idx of edges
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

        Self { config, node_arena: node_arena, edge_arena: Vec::with_capacity(size * 16), position_arena: position_arena, path: Vec::new(), root: 0 }
    }

    fn get_child_idx(&self, edge_idx: usize) -> Option<usize> {
        self.edge_arena[edge_idx].child_node_idx
    }

    pub fn select_best_edge(&self, node_idx: usize) -> Option<usize> {
        info!("select_best_edge: Start");
        let node = &self.node_arena[node_idx];
        let position = &self.position_arena[node.get_data().chess_position_idx];

        let (start, end) = node.get_data().child_edge_range?;
        let visit_count = node.get_data().visits;

        info!("select_best_edge: Node idx: {}, Position idx: {}, Edge idx: {}, {}", node_idx, node.get_data().chess_position_idx, start, end);

        let calc_puct = |edge_idx: usize| -> f32 {
            let edge = &self.edge_arena[edge_idx];
            let (w, d, l) = (edge.mean_value[0], edge.mean_value[1], edge.mean_value[2]);

            let mut exploitation = w - l;
            if position.side_to_move == Color::Black {
                exploitation *= -1.0;
            }

            let mut prior = edge.confidence;
            if node_idx == self.root {
                let mut rng = XorShift64::new(1234);
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
        info!("select_best_edge: Best edge idx: {:?}", idx);
        info!("select_best_edge: End");
        idx
    }

    pub fn get_network_input(&self, node_idx: usize) -> NetworkInputs {
        info!("get_network_input: Start");
        let node = &self.node_arena[node_idx];
        match node {
            MctsNode::PieceSelect { data } => {
                let position = &self.position_arena[data.chess_position_idx];
                info!("get_network_input: Generating tensor data from chess position idx: {}", &data.chess_position_idx);
                info!("get_network_input: End");
                NetworkInputs::from_position(&position, None)
            }
            MctsNode::PieceMove { data, from_sq } => {
                let position = &self.position_arena[data.chess_position_idx];
                info!("get_network_input: Generating tensor data from chess position idx: {}", &data.chess_position_idx);
                info!("get_network_input: End");
                NetworkInputs::from_position(&position, Some(&from_sq))
            }
        }
    }

    pub fn backprop(&mut self, value: [f32; 3]) {
        info!("backprop: Start");
        self.path.iter().for_each(|&idx| {
            trace!("backprop: updating edge idx: {}", idx);
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
        self.path.clear();
        info!("backprop: End");
    }

    fn add_leaf(&mut self, edge_idx: usize) -> Option<usize> {
        info!("add_leaf: Start");
        let edge = &mut self.edge_arena[edge_idx];
        if edge.child_node_idx.is_some() {
            return None;
        }

        let parent_node = &self.node_arena[edge.parent_node_idx];
        match parent_node {
            MctsNode::PieceSelect { data } => {
                let new_node = MctsNode::PieceMove { data: NodeData::new(data.chess_position_idx, edge_idx), from_sq: edge.square };
                info!(
                    "add_leaf: MctsNode::PieceMove added: {:?} at idx: {}, with arena length: {}",
                    &new_node,
                    self.node_arena.len() - 1,
                    self.node_arena.len()
                );
                self.node_arena.push(new_node);
                let node_idx = self.node_arena.len() - 1;
                info!("add_leaf: Node Arena Length: {}", self.node_arena.len());
                edge.child_node_idx = Some(node_idx);
                info!("add_leaf: End");
                return Some(node_idx);
            }
            MctsNode::PieceMove { from_sq, .. } => {
                let mut position = self.position_arena[parent_node.get_data().chess_position_idx].clone();
                let mov = ChessMove::new(*from_sq, edge.square, edge.promotion_piece);

                position.make_move(&mov);
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
                info!("add_leaf: MctsNode::PieceSelect added, Idx: {}", node_idx);
                info!("add_leaf: End");
                return Some(node_idx);
            }
        }
    }

    pub fn node_to_expand(&self) -> usize {
        if let Some(idx) = self.path.last() {
            if let Some(child_idx) = &self.edge_arena[*idx].child_node_idx {
                return *child_idx;
            }
        }
        return 0;
    }

    pub fn get_position(&self, node_idx: usize) -> ChessPosition {
        self.position_arena[self.node_arena[node_idx].get_data().chess_position_idx].clone()
    }

    pub fn make_targets(&mut self) -> TrainingSample {
        info!("make_targets: Start");
        let node = &self.node_arena[0];
        let position = &self.get_position(0);
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
        self.root = best_edge.child_node_idx.unwrap();
        let targets = NetworkLabels { policy: target_policy, value: best_edge.mean_value };
        info!("make_targets: End");
        TrainingSample { inputs, targets }
    }

    pub fn traverse(&mut self) {
        info!("traverse: Start");

        let mut current_node_idx = 0;
        self.path.clear();

        loop {
            debug!("traverse: Path Length: {}", self.path.len());

            // If current node is expanded, push edge
            if let Some(edge_idx) = self.select_best_edge(current_node_idx) {
                info!("traverse: Found edge ({}) for node: {}", edge_idx, current_node_idx);
                self.path.push(edge_idx);
                let next_node_idx = self.edge_arena[edge_idx].child_node_idx;

                if let Some(next_node_idx) = next_node_idx {
                    // If edge has child node, check if terminal
                    info!("traverse: Edge has leaf node. Moving from node idx: {} to {}", current_node_idx, next_node_idx);
                    let node = &self.node_arena[next_node_idx];
                    if node.get_data().is_terminal {
                        info!("traverse: Node is terminal, adding edge to path and breaking");
                        self.path.push(edge_idx);
                        self.backprop(node.get_data().value.unwrap());
                        info!("traverse: End");
                        break;
                    }
                    info!("traverse: Node found at idx: {}", next_node_idx);
                    current_node_idx = next_node_idx;
                } else {
                    info!("traverse: Edge has no leaf node. Creating...");
                    let node_idx = self.add_leaf(edge_idx).unwrap();
                    let node = &self.node_arena[node_idx];
                    if node.get_data().is_terminal {
                        let value = node.get_data().value;
                        info!("traverse: Node is terminal, outcome: {:?}. Backpropogating...", &value.unwrap());
                        info!("traverse: End");
                        self.backprop(value.unwrap());
                        break;
                    }
                }
            // If current node not expanded, path is complete.
            } else {
                // redundant check?
                let node = &self.node_arena[current_node_idx];
                if node.get_data().is_terminal {
                    self.backprop(node.get_data().value.unwrap());
                    self.path.clear();
                }
                info!("traverse: No edges found for node: {}", current_node_idx);
                info!("traverse: End");
                break;
            }
        }
    }

    pub fn get_move(&self) -> Option<ChessMove> {
        let edge = &self.edge_arena[self.select_best_edge(0)?];
        let node = &self.node_arena[edge.parent_node_idx];
        match node {
            MctsNode::PieceMove { from_sq, .. } => {
                if let Some(piece_type) = edge.promotion_piece {
                    return Some(ChessMove::new(*from_sq, edge.square, Some(piece_type)));
                }
                return Some(ChessMove::new(*from_sq, edge.square, None));
            }
            _ => return None,
        }
    }
}

pub fn expand_batch<B: Backend>(mctss: &mut [Mcts], model: ChessTransformer<B>, config: &TrainingConfig, device: &B::Device) {
    info!("expand_batch: Start");

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

    info!("expand_batch: Generating masks and Network inputs");
    mctss.iter().for_each(|game| {
        let mut node_idx = 0;
        if let Some(edge_idx) = game.path.last() {
            node_idx = game.edge_arena[*edge_idx].child_node_idx.unwrap_or(0);
        }

        info!("expand_batch: expanding node idx: {}", node_idx);
        inputs.push(game.get_network_input(node_idx));

        let node = &game.node_arena[node_idx];
        let position_idx = node.get_data().chess_position_idx;
        let position = &game.position_arena[position_idx];
        match node {
            MctsNode::PieceSelect { .. } => {
                info!("expand_batch: generating select mask for position idx: {}", position_idx);
                info!("expand_batch: displaying chessboard for node\n{}", position.chessboard.display_ascii());
                let mask = position.make_mask(config.legal, None);
                info!("expand_batch: displaying mask:\n{}", print_mask(&mask));
                masks.push(mask);
            }
            MctsNode::PieceMove { from_sq, .. } => {
                info!("expand_batch: generating move mask for position idx: {}", position_idx);
                masks.push(position.make_mask(config.legal, Some(*from_sq)));
            }
        }
    });

    debug!("expand_batch: Mask length: {:?}", &masks[0].len());
    debug!("expand_batch: Input length: {:?}", &inputs.len());
    assert_eq!(&inputs.len(), &config.batch_size, "ERROR: Batch size != Network Inputs");

    // generate outputs
    let mut mask_in: Vec<bool> = vec![false; config.batch_size * 64];
    masks.iter().enumerate().for_each(|(i, mask)| mask_in[(i * 64)..(i * 64 + 64)].copy_from_slice(mask));

    info!("expand batch: Running model inference");
    let outputs: Vec<NetworkLabels> = model_make_outputs(model.clone(), &inputs, config, if config.masked { Some(mask_in) } else { None }, device);
    info!("expand batch: Inference done");

    debug!("expand_batch: Policy length: {:?}", &outputs[0].policy.len());
    debug!("expand_batch: value: {:?}", &outputs[0].value);

    mctss.iter_mut().zip(outputs.into_iter()).zip(masks.into_iter()).for_each(|((game, output), mask)| {
        let node_idx = game.node_to_expand();
        let position = &game.get_position(node_idx);
        let node = &mut game.node_arena[node_idx];

        let start = game.edge_arena.len();
        let (policy, value) = (output.as_squares(), output.value);
        info!("expand_batch: Adding edges");
        info!("{}", position);
        info!("{}", print_mask(&mask));
        assert!(mask.iter().any(|&e| e == true) == true, "mask is empty!! wtf?!");
        for i in 0..64 {
            // this means only legal/pseudo legal moves are pushed
            if mask[i] == true {
                match node {
                    MctsNode::PieceMove { from_sq, .. } => {
                        // if move is promotion, add 4 edges, one for each promotion
                        let mov = ChessMove::new(*from_sq, ChessSquare(i as u8), None);
                        // expand_if_prom -> Option<[ChessMove; 4]> turns none into promotions lol
                        if let Some(moves) = position.expand_if_prom(mov) {
                            for mov in moves {
                                info!("expand_batch: Adding promotion edge to MctsNode::PieceMove: {}", &node_idx);
                                let edge = MctsEdge::new(policy[i].0, policy[i].1 / 4.0, node_idx).with_prom(mov.promotion.unwrap());
                                game.edge_arena.push(edge);
                            }
                        } else {
                            info!("expand_batch: Adding edge to MctsNode::PieceMove: {}", &node_idx);
                            let edge = MctsEdge::new(policy[i].0, policy[i].1, game.position_arena.len());
                            game.edge_arena.push(edge);
                        }
                    }
                    MctsNode::PieceSelect { .. } => {
                        info!("expand_batch: Adding edge to MctsNode::PieceSelect: {}", &node_idx);
                        let edge = MctsEdge::new(policy[i].0, policy[i].1, node_idx);
                        game.edge_arena.push(edge);
                    }
                }
            }
        }
        let end = game.edge_arena.len() - 1;
        info!("expand_batch: Added {} edges to Node idx: {}, at ({}, {})", end - start, node_idx, start, end);

        // update node
        node.get_data_mut().child_edge_range = Some((start, end));
        node.get_data_mut().value = Some(value);
        info!("expand_batch: Starting backprop with value {:?}", &value);
        game.backprop(value);
        info!("expand_batch: Finished expanding");
    });
}

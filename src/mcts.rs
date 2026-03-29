use burn::prelude::Backend;

use crate::{
    ChessGame, ChessMove, ChessPosition, ChessSquare, ChessTransformer, Color, NetworkInputs, TrainingConfig,
    chess_game::Outcome, model_make_outputs,
};

#[derive(Default, Debug, Copy, Clone)]
pub struct NodeData {
    chess_position_idx: usize,
    child_edge_idx: Option<(usize, usize)>, // None if un expanded
    parent_edge_idx: Option<usize>,         // None if root
    value: Option<[f32; 3]>,                // None if game outcome unfinished, replace with network value
    is_terminal: bool,
}

impl NodeData {
    pub fn new(chess_position_idx: usize, parent_edge_idx: usize) -> Self {
        Self {
            chess_position_idx,
            child_edge_idx: None,
            parent_edge_idx: Some(parent_edge_idx),
            value: None,
            is_terminal: false,
        }
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
    fn new_select(chess_position_idx: usize, parent_edge_idx: usize) -> Self {
        Self::PieceSelect { data: NodeData::new(chess_position_idx, parent_edge_idx) }
    }

    fn new_move(chess_position_idx: usize, parent_edge_idx: usize, from_sq: ChessSquare) -> Self {
        Self::PieceMove { data: NodeData::new(chess_position_idx, parent_edge_idx), from_sq }
    }

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
        }
    }
}

// --------------------

#[derive(Debug, Default, Copy, Clone)]
pub struct MctsConfig {
    pub num_simulations: usize,
    pub c_puct: f32,
    pub temperature: f32,
}

pub struct Mcts<'a> {
    pub root: &'a ChessGame,
    pub config: MctsConfig,
    pub node_arena: Vec<MctsNode>,
    pub edge_arena: Vec<MctsEdge>,
    pub position_arena: Vec<ChessPosition>,
    pub path: Vec<usize>, // idx of edges
}

impl<'a> Mcts<'a> {
    pub fn new(game: &'a ChessGame, size: usize, config: MctsConfig) -> Self {
        Self {
            root: game,
            config,
            node_arena: Vec::with_capacity(size * 2),
            edge_arena: Vec::with_capacity(size * 16),
            position_arena: Vec::with_capacity(size),
            path: Vec::new(),
        }
    }

    pub fn select_best_edge(&self, node_idx: usize) -> Option<usize> {
        let node = &self.node_arena[node_idx];
        let position = &self.position_arena[node.get_data().chess_position_idx];

        let (start, end) = node.get_data().child_edge_idx?;
        let visit_count: u32 = self.edge_arena[start..=end].iter().map(|e| e.visits).sum();

        let calc_puct = |edge_idx: usize| -> f32 {
            let edge = &self.edge_arena[edge_idx];
            let (w, d, l) = (edge.mean_value[0], edge.mean_value[1], edge.mean_value[2]);
            let mut value = (w - l) / (w + d + l);
            if position.side_to_move == Color::Black {
                value *= -1.0;
            }
            let exploitation = value / visit_count as f32;
            exploitation + self.config.c_puct * (visit_count as f32).sqrt() / (1 + edge.visits) as f32
        };

        let idx = (start..=end).max_by(|&x, &y| {
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
                let position = self.position_arena[data.chess_position_idx].clone();
                NetworkInputs::from_position(&position, None)
            }
            MctsNode::PieceMove { data, from_sq } => {
                let position = self.position_arena[data.chess_position_idx].clone();
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
        })
    }

    pub fn traverse(&mut self) {
        let mut current_edge = 0;
        let mut current_node = 0;
        self.path.clear();
        while let Some(node_idx) = self.edge_arena[current_edge].child_node_idx {
            self.path.push(current_edge);
            current_node = node_idx;
            current_edge = self.select_best_edge(node_idx).unwrap();
        }
        let edge = &mut self.edge_arena[current_edge];
        let node = &self.node_arena[current_node];
        match node {
            MctsNode::PieceSelect { data } => {
                let data = NodeData::new(node.get_data().chess_position_idx, current_edge);
                let node = MctsNode::PieceMove { data, from_sq: edge.square };
                self.node_arena.push(node);
                edge.child_node_idx = Some(self.node_arena.len());
            }
            MctsNode::PieceMove { data, from_sq } => {
                let mut position = self.position_arena[node.get_data().chess_position_idx].clone();
                let mov = ChessMove::new(*from_sq, edge.square, None);
                position.make_move(&mov);
                self.position_arena.push(position);
                let data = NodeData::new(self.position_arena.len(), current_edge);
                let node = MctsNode::PieceSelect { data };
                self.node_arena.push(node);
                edge.child_node_idx = Some(self.node_arena.len());
            }
        }
    }
}

pub fn expand<B: Backend>(games: &mut [Mcts], model: ChessTransformer<B>, config: &TrainingConfig, device: &B::Device) {
    let mut masks: Vec<[bool; 64]> = Vec::with_capacity(config.batch_size * 64);
    let mut inputs: Vec<NetworkInputs> = Vec::with_capacity(config.batch_size);
    games.iter().for_each(|game| {
        let edge_idx = game.path.last().unwrap();
        let edge = &game.edge_arena[*edge_idx];
        let node_idx = edge.child_node_idx.unwrap();
        inputs.push(game.get_network_input(node_idx));
        let node = &game.node_arena[node_idx];
        let position_idx = node.get_data().chess_position_idx;
        let position = &game.position_arena[position_idx];
        match node {
            MctsNode::PieceSelect { .. } => masks.push(position.make_mask(config.legal, None)),
            MctsNode::PieceMove { from_sq, .. } => masks.push(position.make_mask(config.legal, Some(*from_sq))),
        }
    });
    let mask_in: Vec<bool> = masks.clone().into_iter().flatten().collect();
    let outputs =
        model_make_outputs(model.clone(), &inputs, config, if config.masked { Some(mask_in) } else { None }, device);

    games.iter_mut().zip(outputs.into_iter()).zip(masks.into_iter()).for_each(|((game, output), mask)| {
        // fucking what the fuck is this unreadable shit
        let edge_idx = game.path.last().unwrap().clone();
        let start = game.edge_arena.len();
        let edge = &mut game.edge_arena[edge_idx];
        let uh = edge.child_node_idx.clone();
        let node_idx = edge.child_node_idx.unwrap();
        let node = &mut game.node_arena[node_idx];
        let sqs = output.as_squares();
        for i in 0..64 {
            if mask[i] == true {
                let sq = sqs[i];
                let edge = MctsEdge::new(sq.0, sq.1, uh.unwrap());
                game.edge_arena.push(edge);
            }
        }
        let end = game.edge_arena.len();

        // update node
        node.get_data_mut().child_edge_idx = Some((start, end));

        let position = &game.position_arena[node.get_data().chess_position_idx];
        if let Some(value) = position.check_game_state(config.legal).to_f32() {
            node.get_data_mut().value = Some(value);
            node.get_data_mut().is_terminal = true;
        } else {
            node.get_data_mut().value = Some(output.value);
        }
    });
}

use burn::{prelude::Backend}

use crate::{
    ChessGame, ChessMove, ChessPosition, ChessSquare, ChessTransformer, Color, NetworkInputs, TrainingConfig, model_make_outputs
};

pub struct MctsConfig {
    pub num_simulations: usize,
    pub c_puct: f32,
    pub temperature: f32,
}
/*
edge_arena
node_arena
position_arena
*/

pub struct NodeData {
    chess_position_idx: usize,      // idx into Vec<ChessPosition>
    edge_idxs: Option<[usize; 16]>, // can only select from 16 pieces.
    parent_edge_idx: Option<usize>, // None if root.
    value: Option<[f32; 3]>,        // from game.outcome -> network value, else None
}

pub enum MctsNode {
    PieceSelect { data: NodeData },
    PieceMove { selected_sq: ChessSquare, data: NodeData },
}

impl Default for NodeData {
    fn default() -> Self {
        Self {
            chess_position_idx: 0, value: None, edge_idxs: None, parent_edge_idx: None
        }
    }
}

impl NodeData {
    pub fn new(chess_position_idx: usize, parent_edge_idx: usize) -> Self {
        Self {
            chess_position_idx, value: None, edge_idxs: None, parent_edge_idx: Some(parent_edge_idx)
        }
    }

    pub fn get_edges<'a>(&self, edge_arena: &'a [MctsEdge]) -> Option<&'a [MctsEdge]> {
        // idxs are contiguous right? when i expand it pushes all edges sequentially so it must be
        // contiguous right? damn its ugly
        let idxs = self.edge_idxs?;
        let smallest = idxs.iter().min()?;
        let largest = idxs.iter().max()?;
        Some(&edge_arena[*smallest..=*largest])
    }

    fn total_visits(&self, edge_arena: &[MctsEdge]) -> Option<u32> {
        let edges = self.get_edges(edge_arena)?;
        Some(edges.iter().map(|e| e.visits).sum())
    }

    fn select_best_edge(&self, c_puct: f32, side_to_move: Color, edge_arena: &[MctsEdge]) -> Option<usize> {
        let total_visits = self.total_visits(edge_arena)?;
        let edges = self.get_edges(edge_arena)?;
        let edge = edges.iter().max_by(|x, y| {
            let a = x.get_puct(c_puct, &total_visits, side_to_move);
            let b = y.get_puct(c_puct, &total_visits, side_to_move);
            a.total_cmp(&b)
        });

        // *pukes*
        for i in 0..edge_arena.len() {
            if edge_arena[i] == *edge? {
                return Some(i);
            }
        }
        None
    }
}

impl MctsNode {
    fn node_data(&self) -> &NodeData {
        match self {
            Self::PieceSelect { data } => data,
            Self::PieceMove { data, .. } => data,
        }
    }

    fn node_data_mut(&mut self) -> &mut NodeData {
        match self {
            Self::PieceSelect { data } => data,
            Self::PieceMove { data, .. } => data,
        }
    }

    pub fn get_mask(&self, legal: bool, positions: &[ChessPosition]) -> [bool; 64] {
        let mut mask = [false; 64];
        let position = &positions[self.node_data().chess_position_idx];
        let mut moves = position.pseudolegal_moves.clone();
        if legal {
            moves.retain(|mov| position.is_legal(mov));
        }

        match self {
            Self::PieceSelect {..} => {
                moves.iter().for_each(|mov| mask[mov.from.0 as usize] = true);
            }
            Self::PieceMove {selected_sq, ..} => {
                moves.retain(|mov| mov.from == *selected_sq);
                moves.iter().for_each(|mov| mask[mov.to.0 as usize] = true);
            }
        }
        mask
    }

    fn to_network_inputs(&self, positions: &[ChessPosition]) -> NetworkInputs {
        let position = &positions[self.node_data().chess_position_idx];
        match self {
            Self::PieceSelect {..} => {
                NetworkInputs::from_position(position, None)
            }
            Self::PieceMove {  selected_sq,.. } => {
                NetworkInputs::from_position(position, Some(selected_sq))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq )]
pub struct MctsEdge {
    pub square:      ChessSquare,
    pub confidence:  f32, // the policy prob
    pub visits:      u32,
    pub total_value: [f32; 3], // cumulative value of leaf nodes
    pub mean_value:  [f32; 3], // total val / visits
    pub child_node_idx:   Option<usize>, // None if not explored
    pub parent_node_idx:  usize,
}

impl MctsEdge {
    pub fn new(sq: ChessSquare, confidence: f32, parent_idx: usize) -> Self {
        MctsEdge {
            square: sq,
            confidence,
            total_value: [0.0; 3],
            mean_value: [0.0; 3],
            visits: 0,
            child_node_idx: None,
            parent_node_idx: parent_idx,
        }
    }

    pub fn get_puct(&self, c_puct: f32, total_parent_visits: &u32, side_to_move: Color) -> f32 {
        let value = match side_to_move {
            Color::White => self.total_value[0] - self.total_value[2] - self.total_value[1],
            Color::Black => self.total_value[2] - self.total_value[0] - self.total_value[1],
        };
        let exploitation = value / *total_parent_visits as f32;
        let prior = self.confidence;
        // idk lol
        exploitation + c_puct * prior * (*total_parent_visits as f32).sqrt() / (1 + self.visits) as f32
    }

    pub fn update(&mut self, value: [f32; 3]) {
        self.visits += 1;
        self.total_value[0] += value[0];
        self.total_value[1] += value[1];
        self.total_value[2] += value[2];
        // self.total_value.iter_mut().zip(value.iter()).for_each(|(a, b)| *a = *a + b);
        self.mean_value.iter_mut().zip(self.total_value.iter()).for_each(|(a, b)| *a = b / self.visits as f32);
    }

    // is this method supposed to belong here? annoying to get the current idx
    pub fn make_node(&self, nodes: &Vec<MctsNode>, positions: &mut Vec<ChessPosition>) -> MctsNode{
        let node = &nodes[self.parent_node_idx];
        match node {
            MctsNode::PieceSelect { data } => {
                let node = NodeData::new(data.chess_position_idx, current_idx);
                MctsNode::PieceMove { selected_sq: self.square, data: node }
            }
            MctsNode::PieceMove { selected_sq, data } => {
                let parent_node = &nodes[self.parent_node_idx];
                let mut position = positions[parent_node.node_data().chess_position_idx].clone();
                let mov = ChessMove::new(*selected_sq, self.square, None);
                position.make_move(&mov);
                positions.push(position);
                let node = NodeData::new(data.chess_position_idx + 1, current_idx);
                MctsNode::PieceMove { selected_sq: self.square, data: node }
            }
        }
    }
}

pub fn expand<B: Backend>(
    nodes: &mut [MctsNode], // list of leaf nodes to expand.
    positions: &[ChessPosition],
    model: ChessTransformer<B>,
    config: &TrainingConfig,
    device: &B::Device,
) {
    let inputs = nodes.iter().map(|node| node.to_network_inputs(positions)).collect();
    let masks: Option<Vec<bool>> = if config.masked {
        let mut masks: Vec<bool> = Vec::with_capacity(config.batch_size * 64);
        nodes.iter().for_each(|node| {
            masks.extend_from_slice(&node.get_mask(config.legal, positions));
        });
        Some(masks)
    } else {
        None
    };
    // shouldn't be making 64 edges but whatever
    let outputs = model_make_outputs(model.clone(), &inputs, config, masks, device);
    nodes.iter_mut().zip(outputs.iter()).for_each(|(node, output)| {
        let mut sqs = output.as_squares().into_iter();
        let edges: [MctsEdge; 64] =
            std::array::from_fn(|_| sqs.next().map(|e| MctsEdge::new(e.0, e.1)).unwrap_or_default());
        match node {
            MctsNode::PieceSelect { data } => data.edge_idxs = Some(edges),
            MctsNode::PieceMove { data, .. } => data.edge_idxs = Some(edges),
        }
    });
}

pub struct Mcts<'a> {
    pub root: &'a ChessGame,
    pub node_arena: Vec<MctsNode>,
    pub position_arena: Vec<ChessPosition>,
    pub edges_arena: Vec<MctsEdge>,
    pub path: Vec<MctsEdge>,
}

impl<'a> Mcts<'a> {
    pub fn new(game: &'a ChessGame, size: usize) -> Self {
        Mcts{
            root: game,
            node_arena: Vec::with_capacity(size* 2),
            position_arena: Vec::with_capacity(size),
            edges_arena: Vec::with_capacity(size * 16),
            path: Vec::new()
        }
    }
}

pub fn run_mcts<B: Backend>(
    games: &Vec<ChessGame>,
    model: ChessTransformer<B>,
    mcts_config: &MctsConfig,
    training_config: &TrainingConfig,
    device: &B::Device,
) {
    for count in 0..mcts_config.num_simulations {}
}

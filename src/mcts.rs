use burn::prelude::Backend;

use crate::{
    ChessGame, ChessPosition, ChessSquare, ChessTransformer, Color, NetworkInputs, TrainingConfig, model_make_outputs,
};

pub struct MctsConfig {
    pub num_simulations: usize,
    pub c_puct: f32,
    pub temperature: f32,
}

#[derive(Debug, Clone)]
pub struct MctsNodeState {
    position: ChessPosition,
    value:    Option<[f32; 3]>, // from network or outcome
    edges:    Option<Vec<MctsEdge>>,
    parent_edge:   Option<usize>,
}

#[derive(Debug, Clone)]
pub struct MctsNodeAction<'a> {
    position: &'a ChessPosition, // ref to state node position
    value: Option<[f32; 3]>,
    edges: Option<Vec<MctsEdge>>,
    selected_sq: ChessSquare,
    parent_edge: usize,
}

#[derive(Debug, Clone)]
pub struct MctsEdge {
    pub square: ChessSquare,
    pub confidence: f32, // the policy prob
    pub visits: u32,
    pub total_value: [f32; 3], // cumulative value of leaf nodes
    pub mean_value: [f32; 3],  // total val / visits
    pub child_node: Option<usize>,
}

impl MctsNodeState {
    pub fn new(game: &ChessGame, parent: Option<usize>) -> Self {
        Self { position: game.position.clone(), value: None, edges: None, parent_edge: parent }
    }
}

impl<'a> MctsNodeAction<'a> {
    pub fn new(mcts_state: &'a MctsNodeState, selected_sq: ChessSquare, parent: usize) -> Self {
        Self { position: &mcts_state.position, selected_sq, value: None, edges: None, parent_edge: parent }
    }
}

trait Node {
    type Next;
    fn new_node(&self, position: &ChessPosition, parent_edge: usize) -> Self::Next;

    fn position(&self) -> &ChessPosition;
    fn value(&self) -> Option<[f32; 3]>;
    fn edges(&self) -> &Option<Vec<MctsEdge>>;
    fn edges_mut(&mut self) -> &mut Option<Vec<MctsEdge>>;

    fn get_mask(&self, legal: bool) -> [bool; 64];

    fn total_visits(&self) -> Result<u32, ()> {
        if let Some(edges) = self.edges() {
            return Ok(edges.iter().map(|e| e.visits).sum());
        };
        return Err(());
    }

    fn select_best_edge(&self, c_puct: f32, side_to_move: Color) -> Result<&MctsEdge, ()> {
        let total_visits = self.total_visits()?;
        let edges = self.edges().as_ref().unwrap();
        edges.iter().max_by(|x, y| {
            let a = x.get_puct(c_puct, &total_visits, side_to_move);
            let b = y.get_puct(c_puct, &total_visits, side_to_move);
            a.total_cmp(&b)
        }).ok_or(()) // lol i dont distuinguish between None and empty vec. just make sure
                     // i check outcome of node before expanding and should be fine
    }

    fn to_network_inputs(&self) -> NetworkInputs;
}

impl<'a> Node for MctsNodeAction<'a> {
    type Next = MctsNodeState;
    fn new_node(&self, position: &ChessPosition, parent_edge: usize) -> Self::Next {
        MctsNodeState {
            position: position.clone(), value: None, edges: None, parent_edge: Some(parent_edge)
        }
    }
    fn position(&self) -> &'a ChessPosition {
        self.position
    }
    fn value(&self) -> Option<[f32; 3]> {
        self.value
    }
    fn edges_mut(&mut self) -> &mut Option<Vec<MctsEdge>> {
        &mut self.edges
    }
    fn edges(&self) -> &Option<Vec<MctsEdge>> {
        &self.edges
    }
    fn to_network_inputs(&self) -> NetworkInputs {
        NetworkInputs::from_position(self.position(), Some(self.selected_sq))
    }
    fn get_mask(&self, legal: bool) -> [bool; 64] {
        let mut moves = self.position().pseudolegal_moves.clone();
        if legal {
            moves.retain(|mov| self.position().is_legal(mov));
        }
        let sqs: Vec<ChessSquare> = moves.iter().filter(|mov| mov.from == self.selected_sq).map(|mov| mov.to).collect();
        let mut mask = [false; 64];
        sqs.iter().for_each(|sq| mask[sq.0 as usize] = true);
        mask
    }
}

impl Node for MctsNodeState {
    type Next = MctsNodeAction;
    fn new_node(&self, parent_edge: usize) -> Self::Next {
        MctsNodeAction {
            position: &self.position, value: None, edges: None, selected_sq: ChessSquare::default(), parent_edge
        }
    }
    fn position(&self) -> &ChessPosition {
        &self.position
    }
    fn value(&self) -> Option<[f32; 3]> {
        self.value
    }
    fn edges(&self) -> &Option<Vec<MctsEdge>> {
        &self.edges
    }
    fn edges_mut(&mut self) -> &mut Option<Vec<MctsEdge>> {
        &mut self.edges
    }
    fn to_network_inputs(&self) -> NetworkInputs {
        NetworkInputs::from_position(self.position(), None)
    }
    fn get_mask(&self, legal: bool) -> [bool; 64] {
        let mut moves = self.position().pseudolegal_moves.clone();
        if legal {
            moves.retain(|mov| self.position().is_legal(mov));
        }
        let sqs: Vec<ChessSquare> = moves.iter().map(|mov| mov.from).collect();
        let mut mask = [false; 64];
        sqs.iter().for_each(|sq| mask[sq.0 as usize] = true);
        mask
    }
}

impl MctsEdge {
    pub fn new(sq: ChessSquare, confidence: f32) -> Self {
        MctsEdge { square: sq, confidence, total_value: [0.0; 3], mean_value: [0.0; 3], visits: 0, child_node: None }
    }

    pub fn get_puct(&self, c_puct: f32, total_parent_visits: &u32, side_to_move: Color) -> f32 {
        let value = match side_to_move {
            Color::White => self.total_value[0] - self.total_value[2] - self.total_value[1],
            Color::Black => self.total_value[2] - self.total_value[0] - self.total_value[1],
        };
        let exploitation = value / *total_parent_visits as f32;
        let prior = self.confidence;
        // idk lol
        exploitation + c_puct * prior * *total_parent_visits as f32 / (1 + self.visits) as f32
    }

    pub fn update(&mut self, value: [f32; 3]) {
        self.visits += 1;
        self.total_value[0] += value[0];
        self.total_value[1] += value[1];
        self.total_value[2] += value[2];
        // self.total_value.iter_mut().zip(value.iter()).for_each(|(a, b)| *a = *a + b);
        self.mean_value.iter_mut().zip(self.total_value.iter()).for_each(|(a, b)| *a = b / self.visits as f32);
    }
}

pub fn expand<B: Backend>(
    nodes: &mut Vec<impl Node>,
    model: ChessTransformer<B>,
    config: &TrainingConfig,
    device: &B::Device,
) {
    let inputs = nodes.iter().map(|node| node.to_network_inputs()).collect();
    let masks: Option<Vec<bool>> = if config.masked {
        let mut masks: Vec<bool> = Vec::with_capacity(config.batch_size * 64);
        nodes.iter().for_each(|node| {
            masks.extend_from_slice(&node.get_mask(config.legal));
        });
        Some(masks)
    } else {
        None
    };
    // shouldn't be making 64 edges but whatever
    let outputs = model_make_outputs(model.clone(), &inputs, config, masks, device);
    nodes.iter_mut().zip(outputs.iter()).for_each(|(node, output)| {
        if node.edges_mut().is_none() {
            *node.edges_mut() = Some(Vec::with_capacity(64));
        } else {
            return;
        }
        let sqs = output.as_squares();
        for (sq, confidence) in sqs {
            let edge = MctsEdge::new(sq, confidence);
            node.edges_mut().as_mut().unwrap().push(edge);
        }
    })
}

pub struct Mcts<'a> {
    pub root: &'a ChessGame,
    pub state_arena: Vec<MctsNodeState>,
    pub action_arena: Vec<MctsNodeAction<'a>>,
    pub path: Vec<MctsEdge>,
    pub current_idx: usize,
}

impl<'a> Mcts<'a> {
    pub fn new(game: &'a ChessGame) -> Self {
        Self {
            root: game,
            state_arena: Vec::new(),
            action_arena: Vec::new(),
            path: Vec::new(),
                current_idx: 0
        }
    }

    pub fn next(&self) {
    }
}

pub fn run_mcts<B: Backend>(
    games: &Vec<ChessGame>,
    model: ChessTransformer<B>,
    mcts_config: &MctsConfig,
    training_config: &TrainingConfig,
    device: &B::Device,
) {

    for count in 0..mcts_config.num_simulations {

    }
}

use burn::{
    Tensor,
    config::Config,
    optim::{Adam, AdamConfig, GradientsParams, Optimizer, adaptor::OptimizerAdaptor},
    prelude::Backend,
    tensor::{Bool, TensorData, activation::softmax, backend::AutodiffBackend},
};
use rand::rngs::SmallRng;

use crate::{
    ChessGame, ChessPosition, ChessSquare, ChessTransformer, Color, ReplayBuffer,
    chess_game::{Outcome, RuleSet},
    data::{ChessBatch, TrainingSample},
    model::ChessTransformerConfig,
};

#[derive(Config, Debug)]
pub struct TrainingConfig {
    pub model: ChessTransformerConfig,
    pub masked: bool,
    pub optimizer: AdamConfig,
    #[config(default = 10)]
    pub num_epochs: usize,
    #[config(default = 1024)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 1234)]
    pub seed: u64,
    #[config(default = 1.0e-4)]
    pub learning_rate: f64,
}

#[derive(Clone, Copy)]
pub struct NetworkInputs {
    boards: [f32; 64 * 14],
    meta:   [f32; 5],
}

#[derive(Clone, Copy)]
pub struct NetworkOutputs {
    policy: [f32; 64],
    value:  [f32; 3],
}

impl Default for NetworkInputs {
    fn default() -> Self {
        NetworkInputs::from_position(&ChessPosition::default(), None)
    }
}

impl Default for NetworkOutputs {
    fn default() -> Self {
        Self { policy: [0.0; 64], value: [0.0; 3] }
    }
}

impl NetworkInputs {
    pub fn from_position(position: &ChessPosition, selected_sq: Option<ChessSquare>) -> Self {
        let (chess_board, castling_rights, ep_sq) = if position.side_to_move == Color::White {
            (position.chessboard.clone(), position.castling_rights, position.en_passant)
        } else {
            (
                position.chessboard.flip_board(),
                position.castling_rights.flip_perspective(),
                position.en_passant.map(|x| x.square_opposite()),
            )
        };

        let mut data = [0f32; 64 * 14];
        chess_board.pieces[0][0].write_to_slice(&mut data[0..64]);
        chess_board.pieces[0][1].write_to_slice(&mut data[64..128]);
        chess_board.pieces[0][2].write_to_slice(&mut data[128..192]);
        chess_board.pieces[0][3].write_to_slice(&mut data[192..256]);
        chess_board.pieces[0][4].write_to_slice(&mut data[256..320]);
        chess_board.pieces[0][5].write_to_slice(&mut data[320..384]);

        chess_board.pieces[1][0].write_to_slice(&mut data[384..448]);
        chess_board.pieces[1][1].write_to_slice(&mut data[448..512]);
        chess_board.pieces[1][2].write_to_slice(&mut data[512..576]);
        chess_board.pieces[1][3].write_to_slice(&mut data[576..640]);
        chess_board.pieces[1][4].write_to_slice(&mut data[640..704]);
        chess_board.pieces[1][5].write_to_slice(&mut data[704..768]);

        if let Some(square) = ep_sq {
            data[768 + square.0 as usize] = 1.0;
        }

        if let Some(square) = selected_sq {
            data[832 + square.0 as usize] = 1.0;
        }

        let mut meta = [0f32; 5];
        for i in 0..4 {
            meta[i] = (castling_rights.0 >> i & 1).into();
        }
        meta[4] = position.halfmove_clock as f32 / 50.0;

        Self { boards: data, meta }
    }
}

impl NetworkOutputs {
    pub fn as_squares(&self) -> [(ChessSquare, f32); 64] {
        std::array::from_fn(|i| {
            let sq = ChessSquare::new(i as u8).unwrap();
            let val = self.policy[i];
            (sq, val)
        })
    }
}

fn step<B: Backend>(
    model: ChessTransformer<B>,
    games: &Vec<NetworkInputs>,
    batch_size: usize,
    mask: Option<Vec<bool>>,
    device: &B::Device,
) -> Vec<NetworkOutputs> {
    let (board, meta) = to_tensor(games, device);
    let (mut policy, value) = model.forward(board, meta);
    if let Some(bools) = mask {
        let data = TensorData::new(bools, [batch_size, 64]); // scary
        let mask = Tensor::<B, 2, Bool>::from_data(data, device);
        policy = policy.clone().mask_fill(mask, -1e9);
    }
    policy = softmax(policy, 1);
    // batch x 64
    let policy: Tensor<B, 1> = policy.flatten(0, 1);
    let policy = policy.into_data().to_vec::<f32>().unwrap();
    let values: Tensor<B, 1> = value.flatten(0, 1);
    let values = values.into_data().to_vec::<f32>().unwrap();

    let mut out = vec![NetworkOutputs::default(); batch_size];

    for i in 0..batch_size {
        let mut temp = [0.0; 64];
        let mut temp2 = [0.0; 3];
        for j in 0..64 {
            temp[j] = policy[j + i * 64];
            temp2[0] = values[j];
            temp2[1] = values[j + 1];
            temp2[2] = values[j + 2];
        }
        out[i] = NetworkOutputs { policy: temp, value: temp2 };
    }

    out
}

pub fn train<B: AutodiffBackend>(
    model: ChessTransformer<B>,
    optimizer: &mut OptimizerAdaptor<Adam, ChessTransformer<B>, B>, // what... tf?
    config: TrainingConfig,
    games: &ReplayBuffer,
    device: &B::Device,
    rng: &mut SmallRng,
) {
    let datas: ChessBatch<B> = games.sample_batch(config.batch_size, rng, device);
    let output = model.forward_classification(datas);
    let loss = output.loss;
    let grads = loss.backward();
    let grads = GradientsParams::from_grads(grads, &model);
    optimizer.step(config.learning_rate, model, grads);
}

fn create_artifact_dir(artifact_dir: &str) {
    std::fs::remove_dir_all(artifact_dir).ok();
    std::fs::create_dir_all(artifact_dir).ok();
}

pub fn to_tensor<B: Backend>(buffer: &Vec<NetworkInputs>, device: &B::Device) -> (Tensor<B, 3>, Tensor<B, 2>) {
    let n = buffer.len();

    let mut boards = Vec::with_capacity(n * 64 * 14);
    let mut metas = Vec::with_capacity(n * 5);

    for item in buffer {
        boards.extend_from_slice(&item.boards);
        metas.extend_from_slice(&item.meta);
    }

    let t1 = Tensor::<B, 3>::from_floats(boards.as_slice(), device).reshape([n, 64, 14]);
    let t2 = Tensor::<B, 2>::from_floats(metas.as_slice(), device).reshape([n, 5]);
    (t1, t2)
}

// I cannot be bothered with make unmake.
pub enum MctsNode {
    State {
        position: ChessPosition, // can this be zobrist has instead? but, how to make tensors?
        value:    [f32; 3],      // from network output
        edges:    Vec<MctsEdge>,
    },
    Action {
        position: ChessPosition, // duplicate! game doesnt change between from -> to
        from_sq:  ChessSquare,
        value:    [f32; 3],
        edges:    Vec<MctsEdge>,
    },
}

pub struct MctsEdge {
    pub sq: ChessSquare,
    pub confidence: f32,       // the policy weight
    pub total_value: [f32; 3], // cumulative value from children
    pub visits: u32,
    pub child: Option<usize>, // how do i get the right index though. maybe Option<zobrist>?
}

impl MctsNode {
    pub fn new(position: ChessPosition) -> Self {
        Self::State { position, value: [0.0; 3], edges: Vec::new() }
    }

    pub fn to_network_inputs(&self) -> NetworkInputs {
        match self {
            MctsNode::State { position, .. } => NetworkInputs::from_position(position, None),
            MctsNode::Action { position, from_sq, .. } => NetworkInputs::from_position(position, Some(*from_sq)),
        }
    }

    pub fn expand(&mut self) {
        match self {
            MctsNode::State => {}
            MctsNode::Action => {}
        }
    }
}

impl MctsEdge {
    pub fn new(sq: ChessSquare, confidence: f32, total_value: [f32; 3]) -> Self {
        MctsEdge { sq, confidence, total_value, visits: 0, child: None }
    }
}

pub fn generate_self_play_data<B: AutodiffBackend>(
    model: ChessTransformer<B>,
    config: TrainingConfig,
    device: &B::Device,
) -> Vec<TrainingSample> {
    let mut games = vec![ChessGame::default(); config.batch_size];
    let mut samples: Vec<TrainingSample> = Vec::new();
    let mut arena: Vec<MctsNode> = Vec::new();
    let mut network_outputs: Vec<NetworkOutputs> = Vec::with_capacity(config.batch_size);

    for count in 0..100 {
        games.iter().for_each(|game| {
            let outcome = get_outcome_f32(&game);
            let inputs = NetworkInputs::from_game(&game, None);
        });

        let network_inputs: Vec<NetworkInputs> =
            games.iter().map(|game| NetworkInputs::from_game(&game, None)).collect();
        let out = step(model.clone(), &network_inputs, config.batch_size, None, device);
    }
    samples
}

pub fn get_outcome_f32(game: &ChessGame) -> Option<[f32; 3]> {
    match game.check_game_state() {
        Outcome::Finished(Some(Color::White)) => Some([1.0, 0.0, 0.0]),
        Outcome::Finished(None) => Some([0.0, 1.0, 0.0]),
        Outcome::Finished(Some(Color::Black)) => Some([0.0, 0.0, 1.0]),
        _ => None,
    }
}

pub fn run_training_loop<B: AutodiffBackend>(artifact_dir: &str, config: TrainingConfig, device: &B::Device) {
    let model: ChessTransformer<B> = config.model.init(device);
    let optimizer = config.optimizer.init::<B, ChessTransformer<B>>();
    let mut replay_buffer = ReplayBuffer::new(config.batch_size * 64);

    B::seed(device, config.seed);

    loop {
        println!("--- Iteration: {} ---", &counter);
        let mut masks = vec![false; 64 * config.batch_size];

        // if game is over, push value to samples, push sample to buffer, start new game, generate moves
        games.iter_mut().enumerate().for_each(|(i, game)| {
            if let Outcome::Finished(color) = game.check_game_state() {
                match color {
                    Some(Color::White) => {
                        game.outcome = Outcome::Finished(Some(Color::White));
                        samples[i].target_value = [1.0, 0.0, 0.0];
                    }
                    Some(Color::Black) => {
                        game.outcome = Outcome::Finished(Some(Color::Black));
                        samples[i].target_value = [0.0, 0.0, 1.0];
                    }
                    None => {
                        game.outcome = Outcome::Finished(None);
                        samples[i].target_value = [0.0, 1.0, 0.0];
                    }
                }
                replay_buffer.push(&samples[i]);
                *game = ChessGame::default();
                current_game_state[i] = default_state;
            }
            let mut moves = game.generate_pseudolegal();
            if game.rule_set == RuleSet::Legal {
                moves.retain(|mv| game.is_legal(mv));
            }
            moves.into_iter().for_each(|mv| masks[i * 64 + mv.from.0 as usize] = true);
        });

        let masks = if config.masked { Some(masks) } else { None };

        // vec (u8, f32); 64, (f32; 3)
        let outs = step(model.clone(), &current_game_state, config.batch_size, masks, device);

        // pick top square, replace with mcst eventually
        let mut from_squares: Vec<ChessSquare> = Vec::with_capacity(config.batch_size);
        for NetworkOutputs { policy: p, value: v } in outs {
            let (sq, _) = p.iter().enumerate().max_by(|(_, a), (_, b)| a.total_cmp(b)).unwrap();
            from_squares.push(ChessSquare::new(sq as u8).unwrap());
        }

        // let (target_policy, target_value) = mcst::run();

        samples.iter_mut().enumerate().for_each(|(i, sample)| {
            sample.board = current_game_state[i].boards;
            sample.meta = current_game_state[i].meta;
            //sample.target_policy = policy_from_mcst
            //sample.target_value = value_from_mcst
        });
        replay_buffer.buffer.append(&mut samples);

        // create new samples
        let mut samples: Vec<TrainingSample> = Vec::with_capacity(config.batch_size);

        let to_sqs: Vec<u8> = Vec::with_capacity(config.batch_size);
        // SOMETHINGS WRONG

        counter += 1;
    }
}

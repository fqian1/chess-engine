use burn::{
    Tensor,
    config::Config,
    optim::{Adam, AdamConfig, GradientsParams, Optimizer, adaptor::OptimizerAdaptor},
    prelude::Backend,
    tensor::{Bool, TensorData, activation::softmax, backend::AutodiffBackend},
};
use rand::rngs::SmallRng;

use crate::{
    ChessGame, ChessSquare, ChessTransformer, Color, ReplayBuffer,
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

fn create_artifact_dir(artifact_dir: &str) {
    std::fs::remove_dir_all(artifact_dir).ok();
    std::fs::create_dir_all(artifact_dir).ok();
}

pub fn to_tensor<B: Backend>(
    buffer: &Vec<([f32; 64 * 14], [f32; 5])>,
    device: &B::Device,
) -> (Tensor<B, 3>, Tensor<B, 2>) {
    let n = buffer.len();

    let mut boards = Vec::with_capacity(n * 64 * 14);
    let mut metas = Vec::with_capacity(n * 5);

    for item in buffer {
        boards.extend_from_slice(&item.0);
        metas.extend_from_slice(&item.1);
    }

    let t1 = Tensor::<B, 3>::from_floats(boards.as_slice(), device).reshape([n, 64, 14]);
    let t2 = Tensor::<B, 2>::from_floats(metas.as_slice(), device).reshape([n, 5]);
    (t1, t2)
}

pub fn to_f32(
    game: &ChessGame,
    from_sq: Option<&ChessSquare>,
    to_sq: Option<&ChessSquare>,
) -> ([f32; 64 * 14], [f32; 5]) {
    let (chess_board, castling_rights, ep_sq) = if game.side_to_move == Color::White {
        (game.chessboard.clone(), game.castling_rights, game.en_passant)
    } else {
        (
            game.chessboard.flip_board(),
            game.castling_rights.flip_perspective(),
            game.en_passant.map(|x| x.square_opposite()),
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

    if let Some(square) = from_sq {
        data[832 + square.0 as usize] = 1.0;
    }

    if let Some(square) = to_sq {
        data[832 + square.0 as usize] = 1.0;
    }

    let mut meta = [0f32; 5];
    for i in 0..4 {
        meta[i] = (castling_rights.0 >> i & 1).into();
    }
    meta[4] = game.halfmove_clock as f32 / 50.0;

    (data, meta)
}

pub fn train_model<B: AutodiffBackend>(artifact_dir: &str, config: TrainingConfig, device: &B::Device) {
    let model: ChessTransformer<B> = config.model.init(device);
    let optimizer = config.optimizer.init::<B, ChessTransformer<B>>();
    let mut replay_buffer = ReplayBuffer::new(config.batch_size * 64);

    B::seed(device, config.seed);
    let default_state = to_f32(&ChessGame::default(), None, None);
    let mut counter = 0;
    let mut games = vec![ChessGame::default(); config.batch_size];
    let mut samples: Vec<TrainingSample> = Vec::with_capacity(config.batch_size);
    let mut current_game_state: Vec<([f32; 64 * 14], [f32; 5])> = Vec::with_capacity(config.batch_size);
    games.iter_mut().for_each(|game| {
        game.zobrist_hash = game.calculate_hash();
        let state = to_f32(&game, None, None);
        current_game_state.push(state);
    });

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
        for (policies, _) in outs {
            let (sq, _) = policies.iter().max_by(|&a, &b| a.1.total_cmp(&b.1)).expect("engine bug");
            from_squares.push(ChessSquare::new(*sq).unwrap());
        }

        // mcst naturally makes legal target policy, illegal move culled, visit count -> 0.0.
        // difference is, whether or not itll be propogated back.
        // let (target_policy, target_value) = mcst::run();

        samples.iter_mut().enumerate().for_each(|(i, sample)| {
            sample.board = current_game_state[i].0;
            sample.meta = current_game_state[i].1;
            //sample.target_policy = policy_from_mcst
            //sample.target_value = value_from_mcst
        });
        replay_buffer.buffer.append(&mut samples);

        // create new samples
        let mut samples: Vec<TrainingSample> = Vec::with_capacity(config.batch_size);

        let to_sqs: Vec<u8> = Vec::with_capacity(config.batch_size);
        // wait... repeating...

        counter += 1;
    }
}

fn step<B: Backend>(
    model: ChessTransformer<B>,
    games: &Vec<([f32; 64 * 14], [f32; 5])>,
    batch_size: usize,
    mask: Option<Vec<bool>>,
    device: &B::Device,
    // eww why the fuck am i passing raw data around like this
) -> Vec<([(u8, f32); 64], (f32, f32, f32))> {
    // replay buffer must be populated by chess game before calling step
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

    let mut out = vec![([(0, 0.0); 64], (0.0, 0.0, 0.0))];

    for i in 0..batch_size {
        let mut temp = [(0, 0.0); 64];
        let mut temp2 = (0.0, 0.0, 0.0);
        for j in 0..64 {
            temp[j] = (j as u8, policy[j + i * 64]);
            temp2.0 = values[j];
            temp2.1 = values[j + 1];
            temp2.2 = values[j + 2];
        }
        out[i] = (temp, temp2);
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

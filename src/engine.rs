use burn::{
    Tensor,
    config::Config,
    optim::{Adam, AdamConfig, AdamW, GradientsParams, Optimizer, SimpleOptimizer, adaptor::OptimizerAdaptor},
    prelude::Backend,
    tensor::{Bool, backend::AutodiffBackend},
    train::SupervisedTraining,
};
use rand::rngs::SmallRng;

use crate::{
    ChessGame, ChessSquare, ChessTransformer, Color, ReplayBuffer,
    chess_game::Outcome,
    data::{ChessBatch, ChessBatcher, TrainingSample},
    model::ChessTransformerConfig,
};

#[derive(Config, Debug)]
pub struct TrainingConfig {
    pub model: ChessTransformerConfig,
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

pub fn train_model<B: AutodiffBackend>(artifact_dir: &str, config: TrainingConfig, masked: bool, device: &B::Device) {
    let mut model: ChessTransformer<B> = config.model.init(device, masked);
    let mut optimizer = config.optimizer.init::<B, ChessTransformer<B>>();
    let mut replay_buffer = ReplayBuffer::new(config.batch_size * 64);

    B::seed(device, config.seed);
    let mut counter = 0;
    let mut games = vec![ChessGame::default(); config.batch_size];
    games.iter_mut().for_each(|game| game.zobrist_hash = game.calculate_hash());
    let mut samples = vec![TrainingSample::default(); config.batch_size];

    loop {
        println!("--- Iteration: {} ---", &counter);
        // why do i even store outcome in chess game lol
        games.iter_mut().enumerate().for_each(|(i, game)| {
            match game.check_game_state() {
                Outcome::Finished(color) => {
                    match color {
                        Some(Color::White) => {
                            game.outcome = Outcome::Finished(Some(Color::White));
                            samples[i].target_value = 0;
                        }
                        Some(Color::Black) => {
                            game.outcome = Outcome::Finished(Some(Color::Black));
                            samples[i].target_value = 2;
                        }
                        None => {
                            game.outcome = Outcome::Finished(None);
                            samples[i].target_value = 1;
                        }
                    }
                    replay_buffer.push(samples[i]);
                    *game = ChessGame::default();
                }

                Outcome::Unfinished => {
                    let (board, meta) = game.to_f32(); // this already flips the board if black!!!
                    replay_buffer.buffer[i].board = board;
                    replay_buffer.buffer[i].meta = meta;
                    // need to collect
                }
            }
        });

        counter += 1;
    }
}

fn get_square<B: Backend>(
    model: ChessTransformer<B>,
    replay_buffer: &ReplayBuffer,
    masked: Option<Vec<[bool;64]>>,
    temperature: f32,
    device: &B::Device,
) -> Vec<u8> {
    let (board, meta) = replay_buffer.to_tensor::<B>(device);
    let (policy, value) = model.forward(board, meta);
    if let Some(bools) = masked {
        let mask = Tensor::<B, 2, Bool>::from_bool(bools.into(), device);
        policy.mask_fill(mask, -1e9);
    }
    let [batch_size, _] = policy.dims();
    // batch x 64
    let t_board: Tensor<B, 1> = policy.flatten(1, 2);
    let floats = t_board.into_data().to_vec::<f32>().unwrap();

    let mut squares = vec![0; 64 * batch_size];
    for i in 0..batch_size {
        for j in 0..64 {
            squares[i + j] = j as u8;
        }
        let start = i * 64;
        let end = start + 64;
        let slice = &mut squares[start..end];
        slice.sort_unstable_by(|&a, &b| floats[a as usize].total_cmp(&floats[b as usize]));
    }

    squares

    //         let policy_sq = match rule_set {
    //                 legal => {
    //                         let mut indices: [usize; 64] = [0; 64];
    //                         for i in 0..64 {
    //                                 squares[i] = i
    //                         }
    //
    //                         indices.sort_unstable_by(|&a, &b| {value[b].total_cmp(&value[a])})
    //
    //                         squares: Vec<ChessSquare> = indices.iter().map(|x| ChessSquare::from(x).unwrap())
    //                         squares.filter(self.legal_from_sq)
    //                         top_sq
    //                 }
    //                 pseudo-legal => {
    //                         let max_idx = values.iter().enumerate().fold(0, |acc, (i, x)| {
    //                                 if x > &values[acc] { i } else { acc }
    //                         });
    //                         ChessSquare::from(max_idx)
    //                 }
    //         }
}

pub fn train<B: AutodiffBackend>(
    model: ChessTransformer<B>,
    optimizer: &mut OptimizerAdaptor<Adam, ChessTransformer<B>, B>, // why is this so fukin cursed
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
// loop {
//         let tensors = remaining_batch.iter_mut().for_each(|game| {
//                 let board, meta = if game.side_to_move == black {
//                         game.board.flip, game.meta.to_tensor
//                 } else { game.board, game.meta}
//
//                 let board_tensor = board.to_tensor
//                 let meta_tensor = meta.to_tensor + Tensor::from(half_move_clock/50)
//
//                 (board_tensor, meta_tensor)
//         })
//         if masking, get legal from squares mask output. gen legal from sqs, -> tensor where 0 -> f32::MIN, apply onto policy head, then softmax.
//         if not masking, just skip that
//         let outputs = transformer.forward(board_tensor, meta_tensor) // replace with search tree - take value from depth, but obviously next policy/moves_left
//         // unbatch and turn into Vec<(policy: [f32;64], value: f32); batch_size>
//
//
//         batch.iter_mut().for_each(|game| {
//                 if let Some(entry) = game.GameStateEntry.last_mut() {
//                         entry.value = value
//                 }
//                 game.GameStateEntry.push(GameStateEntry::new(..policy_sq))
//         })
//
//         make tensors again, but include from_sq from last policy
//
//         do forward pass again
//         populate entries
//
//         if needs promotion square, make tensors again {
//                 forward pass again
//                 extract piece:
//                 map policy_sq {
//                         to_sq = q
//                         to_sq - 8 = r // square below promotion sq
//                         to_sq - 16 = b // 2 squares below
//                         to_sq - 24 = n // 3 below
//                         _ => lose if pseudo legal or softmax it out
//                 }
//                 // promotion happen on same file so do like this
//                 populate entries
//         }
//
//         remaining_batch.map(|game| game.make_move) // this should fill in rest of game.GameStateEntry
// }
//
// loop {
//     ChessGame::fen_to_ascii(&game.to_fen());
//     println!("{:?}'s turn.", game.side_to_move);
//
//     print!("Enter move (e.g., e2e4): ");
//     io::stdout().flush().unwrap();
//
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let input = input.trim().to_lowercase();
//
//     if input == "quit" || input == "exit" {
//         break;
//     }
//
//     if input == "debug" {
//         println!("{game:?}");
//     }
//
//     let input = game.uci_to_move(&input);
//     match input {
//         Ok(input) => game.make_move(&input),
//         Err(e) => println!("{e}"),
//     }
// }
//
// impl<B: Backend> SelfPlayRunner<B> {
//     pub fn new(model: ChessTransformer<B>, device: B::Device) -> Self {
//         Self { model, device }
//     }
//
//     pub fn collect_data(&mut self, games: &mut Vec<ChessGame>, buffer: &mut ReplayBuffer, masking_enabled: bool) {
//         let (boards, meta) = crate::bridge::games_to_tensors(games, &self.device);
//         let (policy_logits, _values) = self.model.forward(boards, meta);
//         let logits_data: Vec<Vec<f32>> = policy_logits.to_data().convert().value;
//
//         for (i, game) in games.iter_mut().enumerate() {
//             if matches!(game.outcome, Outcome::Finished(_)) {
//                 *game = ChessGame::default();
//                 continue;
//             }
//
//             let mut logits = logits_data[i].clone();
//
//             // Generate legal moves to check validity
//             game.generate_pseudolegal();
//             let legal_from_squares = game.get_possible_from_squares();
//
//             // --- EXPERIMENT LOGIC ---
//             if masking_enabled {
//                 // MASKING: Set illegal moves to -Infinity so Softmax makes them 0% prob
//                 for sq_idx in 0..64 {
//                     let sq = ChessSquare::new(sq_idx as u8).unwrap();
//                     if !legal_from_squares.contains(&sq) {
//                         logits[sq_idx] = f32::MIN;
//                     }
//                 }
//             }
//             // IF PUNISHMENT: We do nothing to logits. The model might pick an illegal square.
//
//             // Softmax & Sampling
//             let probs = softmax(&logits);
//             let selected_idx = sample_index(&probs);
//             let selected_sq = ChessSquare::new(selected_idx as u8).unwrap();
//
//             // Check Legality
//             let is_legal = legal_from_squares.contains(&selected_sq);
//
//             // Save Sample
//             let (board_f32, meta_f32) = game.to_f32();
//
//             // We don't know the true value yet (Win/Loss), so we use a placeholder.
//             // In a real engine, you backpropagate the result later.
//             // For this specific experiment, immediate punishment is key.
//             let mut reward = 0.0;
//
//             if !is_legal {
//                 // PUNISHMENT: Immediate Loss
//                 reward = -1.0;
//                 game.outcome = Outcome::Finished(Some(game.side_to_move.opposite()));
//             } else {
//                 // LEGAL: Make the move (Random 'To' square for this simplified example)
//                 let possible_tos = game.get_possible_to_squares(&selected_sq);
//                 if let Some(to_sq) = possible_tos.choose(&mut rand::thread_rng()) {
//                     let mv = crate::ChessMove::new(selected_sq, *to_sq, None);
//                     game.make_move(&mv);
//                     game.outcome = game.check_game_state();
//                 }
//             }
//
//             buffer.push(TrainingSample {
//                 board: board_f32,
//                 meta: meta_f32,
//                 target_policy: selected_idx,
//                 target_value: reward,
//             });
//         }
//     }
// }

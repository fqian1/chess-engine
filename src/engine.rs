use std::fs::OpenOptions;
use std::io::Write;

use burn::{
    Tensor,
    config::Config,
    module::{AutodiffModule, Module},
    optim::{AdamW, AdamWConfig, GradientsParams, Optimizer, adaptor::OptimizerAdaptor},
    prelude::{Backend, ToElement},
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
    tensor::{Bool, TensorData, activation::softmax, backend::AutodiffBackend},
};
use log::info;
use rand::{SeedableRng, rngs::SmallRng};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::prelude::*;

use crate::{
    ChessGame, ChessTransformer, Mcts, MctsConfig, ReplayBuffer,
    chess_game::Outcome,
    data::{ChessBatch, NetworkInputs, NetworkLabels},
    expand_batch,
    model::ChessTransformerConfig,
};

fn create_artifact_dir(artifact_dir: &str) {
    std::fs::remove_dir_all(artifact_dir).ok();
    std::fs::create_dir_all(artifact_dir).ok();
}

#[derive(Config, Debug)]
pub struct TrainingConfig {
    pub model: ChessTransformerConfig,
    pub masked: bool,
    pub legal: bool,
    pub optimizer: AdamWConfig,
    #[config(default = 5)]
    pub num_epochs: usize,
    #[config(default = 1024)]
    pub steps_per_iter: usize,
    #[config(default = 256)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 1234)]
    pub seed: u64,
    #[config(default = 1.0e-4)]
    pub learning_rate: f64,
}

pub fn model_make_outputs<B: Backend>(
    model: ChessTransformer<B>,
    inputs: &Vec<NetworkInputs>,
    config: &TrainingConfig,
    masks: Option<Vec<bool>>,
    device: &B::Device,
) -> Vec<NetworkLabels> {
    let batch_size = config.batch_size;
    let (boards, metas) = inputs_to_tensor(inputs, device);
    let (mut policies, mut values) = model.forward(boards, metas);
    if let Some(masks) = masks {
        let mask_data = TensorData::new(masks, [batch_size, 64]);
        let mask = Tensor::<B, 2, Bool>::from_data(mask_data, device);
        let mask = mask.bool_not();

        policies = policies.clone().mask_fill(mask, -1e9);
    }
    policies = softmax(policies, 1);
    values = softmax(values, 1);
    // batch_size x 64
    let policies: Tensor<B, 1> = policies.flatten(0, 1);
    let policies = policies.into_data().to_vec::<f32>().unwrap();

    // batch_size x 3
    let values: Tensor<B, 1> = values.flatten(0, 1);
    let values = values.into_data().to_vec::<f32>().unwrap();

    let mut out: Vec<NetworkLabels> = Vec::with_capacity(batch_size);

    for i in 0..batch_size {
        let policy: [f32; 64] = policies[(i * 64)..(i * 64 + 64)].try_into().unwrap();
        let value: [f32; 3] = values[(i * 3)..(i * 3 + 3)].try_into().unwrap();
        out.push(NetworkLabels { policy, value });
    }

    out
}

pub fn inputs_to_tensor<B: Backend>(buffer: &Vec<NetworkInputs>, device: &B::Device) -> (Tensor<B, 3>, Tensor<B, 2>) {
    let n = buffer.len();

    let mut boards = Vec::with_capacity(n * 64 * 14);
    let mut metas = Vec::with_capacity(n * 5);

    for item in buffer {
        boards.extend_from_slice(&item.boards);
        metas.extend_from_slice(&item.meta);
    }

    let shape = [n, 64, 14];
    let board_data = TensorData::new(boards, shape);
    let t1 = Tensor::from_data(board_data, device);

    let shape = [n, 5];
    let meta_data = TensorData::new(metas, shape);
    let t2 = Tensor::from_data(meta_data, device);

    (t1, t2)
}

pub fn play<B: AutodiffBackend>(artifact_dir: &str, mcts_config: &MctsConfig, training_config: &TrainingConfig, device: &B::Device) {
    create_artifact_dir(artifact_dir);
    B::seed(device, training_config.seed);

    let mut model: ChessTransformer<B> = training_config.model.init(device);
    let mut replay_buffer = ReplayBuffer::new(256000);
    let mut optimizer = training_config.optimizer.init::<B, ChessTransformer<B>>();
    let mut games = vec![ChessGame::default(); training_config.batch_size];
    let mut mctss: Vec<Mcts> = games.iter().map(|game| Mcts::from_game(&game, 1024, *mcts_config)).collect();
    let mut rng = SmallRng::seed_from_u64(training_config.seed);
    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();

    let csv_path = format!("{}/metrics.csv", artifact_dir);
    let mut csv_file = OpenOptions::new().create(true).append(true).open(&csv_path).unwrap();
    if std::fs::metadata(&csv_path).unwrap().len() == 0 {
        writeln!(csv_file, "iteration,avg_loss,avg_game_length,nodes_expanded,avg_illegal_prob").unwrap();
    }

    let mut iterations = 0;
    let mut positions_expanded: u32 = 0;
    let mut game_over_count: f32 = 0.0;
    let mut average_game_length: f32 = 0.0;
    loop {
        info!("Starting Self play - Train loop: cycle {}", iterations);

        let mut illegal_move_weight: f64 = 0.0;

        for _ in 0..training_config.steps_per_iter {
            let (total_length, total_games): (f32, f32) = games
                .par_iter_mut()
                .zip(mctss.par_iter_mut())
                .map(|(game, mcts)| {
                    if let Outcome::Finished(_) = game.check_game_state(training_config.legal) {
                        let length = game.game_history.len() as f32;
                        *game = ChessGame::default();
                        *mcts = Mcts::from_game(&game, 1024, *mcts_config);
                        return (length, 1.0);
                    }
                    (0.0, 0.0)
                })
                .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));

            average_game_length += total_length;
            game_over_count += total_games;

            for _count in 0..mcts_config.num_simulations {
                mctss.par_iter_mut().for_each(|mcts| {
                    mcts.traverse_get_terminal();
                });
                let (unique, weight) = expand_batch(&mut mctss[..], model.clone().valid(), training_config, device);
                illegal_move_weight += weight;
                positions_expanded += unique;
            }

            // get best move and play it
            let new_samples: Vec<_> = mctss
                .par_iter_mut()
                .zip(games.par_iter_mut())
                .map(|(mcts, game)| {
                    let sample = mcts.make_targets();
                    if let Some(mov) = mcts.get_move_to_play() {
                        game.make_move(&mov);
                        info!("\n------\n{}", game.position);
                        info!("Selected move: {}\n------", &mov.to_uci());
                    };
                    mcts.add_dirichlet_noise(mcts.root);
                    sample
                })
                .collect();

            for sample in new_samples {
                if let Some(sample) = sample {
                    replay_buffer.push(sample);
                }
            }

            // info!("Replay Buffer size: {}", replay_buffer.buffer.len());
        }

        let mut loss_val: f32 = 0.0;
        for epoch in 0..training_config.num_epochs {
            info!("Training model: epoch {}", epoch);
            let (new_model, val) = train(model.clone(), &mut optimizer, &training_config, &replay_buffer, device, &mut rng);
            model = new_model;
            loss_val += val;
        }
        loss_val /= training_config.num_epochs as f32;

        let total_batches = (training_config.steps_per_iter * mcts_config.num_simulations) as f64;
        let avg_illegal_prob = illegal_move_weight / total_batches;

        writeln!(csv_file, "{},{},{},{},{}", iterations, loss_val, average_game_length / game_over_count, positions_expanded, avg_illegal_prob)
            .unwrap();
        csv_file.flush().unwrap();

        if iterations % 25 == 0 {
            let snapshot_path = format!("{}/model-{}", artifact_dir, iterations / 25);
            info!("Saving model snapshot at: {}", &snapshot_path);
            if let Err(err) = model.clone().save_file(snapshot_path, &recorder) {
                println!("failed to save model: {}", err);
            }
        }

        iterations += 1;
    }
}

pub fn train<B: AutodiffBackend>(
    model: ChessTransformer<B>,
    optimizer: &mut OptimizerAdaptor<AdamW, ChessTransformer<B>, B>,
    config: &TrainingConfig,
    games: &ReplayBuffer,
    device: &B::Device,
    rng: &mut SmallRng,
) -> (ChessTransformer<B>, f32) {
    let datas: ChessBatch<B> = games.sample_batch(config.batch_size, rng, device);
    let output = model.forward_classification(datas);
    let loss = output.loss;
    let grads = loss.backward();
    let grads = GradientsParams::from_grads(grads, &model);

    let loss_val = loss.clone().into_scalar().to_f32();
    (optimizer.step(config.learning_rate, model.clone(), grads), loss_val)
}

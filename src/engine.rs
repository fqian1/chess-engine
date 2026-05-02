use burn::data::dataloader::batcher::{Batcher};
use burn::record::{FullPrecisionSettings, NamedMpkFileRecorder, Recorder};
use burn::{
    Tensor,
    config::Config,
    lr_scheduler::{
        LrScheduler,
        noam::{NoamLrScheduler, NoamLrSchedulerConfig},
    },
    module::{AutodiffModule, Module},
    optim::{AdamW, AdamWConfig, GradientsParams, Optimizer, adaptor::OptimizerAdaptor},
    prelude::{Backend, ToElement},
    tensor::{Bool, TensorData, activation::softmax, backend::AutodiffBackend},
};
use log::{debug, info, trace};
use rand::seq::IndexedRandom;
use rand::{SeedableRng, rngs::SmallRng};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::{ChessBatcher, Color, TrainingSample};
use crate::{
    ChessGame, ChessTransformer, Mcts, MctsConfig, ReplayBuffer,
    chess_game::Outcome,
    data::{ChessBatch, NetworkInputs, NetworkLabels},
    expand_batch,
    model::ChessTransformerConfig,
};

#[derive(Config, Debug)]
pub struct TrainingConfig {
    pub model: ChessTransformerConfig,
    pub masked: bool,
    pub legal: bool,
    pub scheduler: NoamLrSchedulerConfig,
    pub optimizer: AdamWConfig,
    #[config(default = 256)]
    pub gradient_steps: usize,
    #[config(default = 256)]
    pub steps_per_iter: usize,
    #[config(default = 256)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 1234)]
    pub seed: u64,
}

pub fn model_make_outputs<B: Backend>(
    model: ChessTransformer<B>,
    inputs: &Vec<NetworkInputs>,
    config: &TrainingConfig,
    masks: Vec<bool>,
    device: &B::Device,
) -> Vec<NetworkLabels> {
    let batch_size = config.batch_size;
    let (boards, metas) = inputs_to_tensor(inputs, device);
    let (mut policies, mut values) = model.forward(boards, metas);

    let mask_data = TensorData::new(masks, [batch_size, 64]);
    let mask = Tensor::<B, 2, Bool>::from_data(mask_data, device);
    policies = policies.clone().mask_fill(mask.bool_not(), -1e9);

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

pub fn play<B: AutodiffBackend>(path_arg: &PathBuf, mcts_config: &MctsConfig, training_config: &TrainingConfig, device: &B::Device) {
    let mut model: ChessTransformer<B> = training_config.model.init(device);

    let artifact_dir = if path_arg.is_file() {
        path_arg.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf()
    } else {
        path_arg.clone()
    };

    if !artifact_dir.exists() {
        println!("Creating directory: {:?}", artifact_dir);
        std::fs::create_dir_all(&artifact_dir).expect("Failed to create artifact directory");
    }

    if path_arg.exists() && (path_arg.is_file() || path_arg.to_str().is_some_and(|s| s.ends_with(".mpk"))) {
        println!("Loading model from: {:?}", path_arg);

        let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::default();

        let load_path = if path_arg.extension().is_some_and(|s| s == "mpk") {
            &path_arg.with_extension("")
        } else {
            path_arg
        };

        let record = recorder.load(load_path.into(), device).expect("Failed to load existing model record");

        model = model.load_record(record);
    }
    B::seed(device, training_config.seed);

    let mut replay_buffer = ReplayBuffer::new(524288);
    let mut lr_scheduler = training_config.scheduler.init().unwrap();
    let mut optimizer = training_config.optimizer.init::<B, ChessTransformer<B>>();
    let mut games = vec![ChessGame::default(); training_config.batch_size];
    let mut mctss: Vec<Mcts> = games.iter().map(|game| Mcts::from_game(game, 16384, *mcts_config)).collect();
    let mut rng = SmallRng::seed_from_u64(training_config.seed);
    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();

    let csv_path = format!("{}/metrics.csv", artifact_dir.to_str().unwrap_or("./tmp"));
    let mut csv_file = OpenOptions::new().create(true).append(true).open(&csv_path).unwrap();
    if std::fs::metadata(&csv_path).unwrap().len() == 0 {
        writeln!(csv_file, "iteration,games_started,avg_loss,avg_game_length,wins,draws,nodes_expanded,avg_illegal_prob").unwrap();
    }

    model = pretrain(model.clone(), &mut optimizer, &mut lr_scheduler, training_config, device, &mut rng, &PathBuf::from("/home/fqian/downloads/mate_evals.tsv")).unwrap();

    let mut games_started: u32 = games.len() as u32;
    let mut iterations = 0;
    let mut positions_expanded: u32 = 0;
    let mut wins = 0.0;
    let mut draws = 0.0;
    let mut average_game_length: f32 = 0.0;
    loop {
        info!("Starting Self play - Train loop: cycle {}", iterations);

        let mut illegal_move_weight: f64 = 0.0;

        for _ in 0..training_config.steps_per_iter {
            let (total_length, win, draw, new_games): (f32, f32, f32, u32) = games
                .par_iter_mut()
                .zip(mctss.par_iter_mut())
                .map(|(game, mcts)| {
                    if let Outcome::Finished(color) = game.check_game_state(training_config.legal) {
                        let length = game.game_history.len() as f32;
                        let (win, draw) = if color.is_none() { (0.0, 1.0) } else { (1.0, 0.0) };
                        let new_game = 1;
                        *game = ChessGame::default();
                        info!("starting new game");
                        mcts.refresh(game);
                        return (length, win, draw, new_game);
                    }
                    (0.0, 0.0, 0.0, 0)
                })
                .reduce(|| (0.0, 0.0, 0.0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3));

            games_started += new_games;
            average_game_length += total_length;
            wins += win;
            draws += draw;

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
                    let sample = mcts.make_targets(training_config.masked);
                    if let Some(mov) = mcts.get_move_to_play() {
                        game.make_move(&mov);
                        debug!("\n{}", game.position);
                        trace!("\nSelected move: {}", &mov.to_uci());
                    };
                    // scale draw threshold down after 60 moves
                    let draw_threshold = if game.game_history.len() > 60 { 0.75 } else { 0.95 };
                    if sample.1[1] > draw_threshold || game.game_history.len() > 400 {
                        // sample.1 is root value after search, just restart.
                        game.position.halfmove_clock = 200;
                    }
                    mcts.add_dirichlet_noise(mcts.root);
                    sample
                })
                .collect();

            for (sample, _) in new_samples {
                if let Some(sample) = sample {
                    trace!("{}", sample);
                    replay_buffer.push(sample);
                }
            }

            trace!("Replay Buffer size: {}", replay_buffer.buffer.len());
        }

        let total_batches = (training_config.steps_per_iter * mcts_config.num_simulations) as f64;
        let avg_illegal_prob = illegal_move_weight / total_batches;

        let mut loss_total = Tensor::<B, 1>::zeros([1], device);
        for epoch in 0..training_config.gradient_steps {
            info!("Training model: epoch {}", epoch);
            let (new_model, val) =
                train(model.clone(), &mut optimizer, &mut lr_scheduler, training_config, &replay_buffer, device, avg_illegal_prob as f32, &mut rng);
            model = new_model;
            loss_total = loss_total + val;
        }

        mctss.par_iter_mut().for_each(|mcts| {
            mcts.garbage_collect();
        });

        let loss_val = loss_total / training_config.gradient_steps as f32;
        let loss_val = loss_val.into_scalar().to_f32();

        writeln!(
            csv_file,
            "{},{},{},{},{},{},{},{}",
            iterations,
            games_started,
            loss_val,
            average_game_length / (wins + draws),
            wins,
            draws,
            positions_expanded,
            avg_illegal_prob
        )
        .unwrap();
        csv_file.flush().unwrap();

        if iterations % 25 == 0 {
            let snapshot_index = (iterations / 25) % 10;
            let model_path = format!("{}/model-{}", artifact_dir.to_str().unwrap(), snapshot_index);
            let optim_path = format!("{}/optim-{}", artifact_dir.to_str().unwrap(), snapshot_index);

            info!("Saving model snapshot at: {}", &model_path);

            if let Err(err) = model.clone().save_file(model_path, &recorder) {
                eprintln!("failed to save model: {}", err);
            }

            if let Err(err) = recorder.record(optimizer.to_record(), optim_path.into()) {
                eprintln!("failed to save optimiser: {}", err);
            }
        }

        iterations += 1;
    }
}

#[allow(clippy::too_many_arguments)]
pub fn train<B: AutodiffBackend>(
    model: ChessTransformer<B>,
    optimizer: &mut OptimizerAdaptor<AdamW, ChessTransformer<B>, B>,
    scheduler: &mut NoamLrScheduler,
    config: &TrainingConfig,
    games: &ReplayBuffer,
    device: &B::Device,
    avg_illegal_prob: f32,
    rng: &mut SmallRng,
) -> (ChessTransformer<B>, Tensor<B, 1>) {
    let datas: ChessBatch<B> = games.sample_batch(config.batch_size, rng, device);
    let lr = scheduler.step();

    let output = model.forward_classification(datas, avg_illegal_prob);
    let loss = output.loss;
    let grads = loss.backward();
    let grads = GradientsParams::from_grads(grads, &model);

    (optimizer.step(lr, model.clone(), grads), loss)
}

pub fn pretrain<B: AutodiffBackend>(
    model: ChessTransformer<B>,
    optimizer: &mut OptimizerAdaptor<AdamW, ChessTransformer<B>, B>,
    scheduler: &mut NoamLrScheduler,
    config: &TrainingConfig,
    device: &B::Device,
    rng: &mut SmallRng,
    path: &PathBuf,
) -> io::Result<ChessTransformer<B>> {
    let file = std::fs::read_to_string(path)?;
    let mut samples = Vec::with_capacity(50000);

    for line in file.lines() {
        if samples.len() > 50000 {
            break;
        }
        let parts: Vec<&str> = line.split('\t').collect();

        let fen = parts[0].trim();
        let eval = parts[1].trim();

        let game = match ChessGame::from_fen(fen) {
            Ok(g) => g,
            Err(_) => continue,
        };

        let eval = match eval {
            "-1" | "-2" => {
                if game.position.side_to_move == Color::Black {
                    [1.0, 0.0, 0.0]
                } else {
                    [0.0, 0.0, 1.0]
                }
            }
            "1" | "2" => {
                if game.position.side_to_move == Color::White {
                    [1.0, 0.0, 0.0]
                } else {
                    [0.0, 0.0, 1.0]
                }
            }
            _ => [0.33, 0.33, 0.33], // should be unreachable
        };

        let inputs = NetworkInputs::from_position(&game.position, None);
        let targets = NetworkLabels { policy: [0.0; 64], value: eval };

        let mask = [false; 64]; 

        samples.push(TrainingSample { inputs, targets, mask });
    }

    let batcher = ChessBatcher {};

    for i in 0..100 {
        let lr = scheduler.step();
        let samples: Vec<&TrainingSample> = samples.sample(rng, config.batch_size).collect();
        let batch = batcher.batch(samples.clone(), device);
        let output = model.forward_classification(batch, -1.0);
        let loss = output.loss;
        let grads = loss.backward();
        let grads = GradientsParams::from_grads(grads, &model);
        optimizer.step(lr, model.clone(), grads);
        info!("pre training: {}", i);
    }

    Ok(model)
}

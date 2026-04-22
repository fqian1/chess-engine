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
use log::info;
use rand::{SeedableRng, rngs::SmallRng};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::prelude::*;
use std::fs::{OpenOptions};
use std::io::Write;

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
    let mut model: ChessTransformer<B> = training_config.model.init(device);

    if std::fs::exists(&artifact_dir).expect("failed to read fs") {
        println!("found model in {}", &artifact_dir);
        let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::default();
        let record = recorder.load(artifact_dir.into(), device).expect("Failed to load existing model record");

        model = model.load_record(record);
    } else {
        println!("creating directory: {}", artifact_dir);
        std::fs::create_dir_all(&artifact_dir).expect("Failed to create artifact directory");
    }

    B::seed(device, training_config.seed);

    let mut replay_buffer = ReplayBuffer::new(524288);
    let mut lr_scheduler = training_config.scheduler.init().unwrap();
    let mut optimizer = training_config.optimizer.init::<B, ChessTransformer<B>>();
    let mut games = vec![ChessGame::default(); training_config.batch_size];
    let mut mctss: Vec<Mcts> = games.iter().map(|game| Mcts::from_game(&game, 16384, *mcts_config)).collect();
    let mut rng = SmallRng::seed_from_u64(training_config.seed);
    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();

    let csv_path = format!("{}/metrics.csv", artifact_dir);
    let mut csv_file = OpenOptions::new().create(true).append(true).open(&csv_path).unwrap();
    if std::fs::metadata(&csv_path).unwrap().len() == 0 {
        writeln!(csv_file, "iteration,games_started,avg_loss,avg_game_length,wins,draws,nodes_expanded,avg_illegal_prob").unwrap();
    }

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
            // if let Some(thing) = games.iter().find(|game| matches!(game.check_game_state(training_config.legal), Outcome::Finished(_))) {
            //     thing.game_history.iter().for_each(|pos| {
            //         info!("{}", pos);
            //     });
            // }

            let (total_length, win, draw, new_games): (f32, f32, f32, u32) = games
                .par_iter_mut()
                .zip(mctss.par_iter_mut())
                .map(|(game, mcts)| {
                    if game.position.halfmove_clock > 45 {
                        info!("halfmove clock: {}", game.position.halfmove_clock);
                    }
                    if let Outcome::Finished(color) = game.check_game_state(training_config.legal) {
                        let length = game.game_history.len() as f32;
                        let (win, draw) = if color.is_none() { (0.0, 1.0) } else { (1.0, 0.0) };
                        let new_game = 1;
                        *game = ChessGame::default();
                        *mcts = Mcts::from_game(&game, 10000, *mcts_config);
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
                    let sample = mcts.make_targets();
                    if let Some(mov) = mcts.get_move_to_play() {
                        game.make_move(&mov);
                        info!("\n{}", game.position);
                        // info!("\nSelected move: {}", &mov.to_uci());
                    };
                    // scale draw threshold down after 60 moves
                    let draw_threshold = if game.game_history.len() > 60 { 0.75 } else { 0.95 };
                    if sample.1[1] > draw_threshold || game.game_history.len() > 400 {
                        // sample.1 is root value after search, just restart.
                        game.position.halfmove_clock = 200;
                        // info!("{}", sample.1[1]);
                    }
                    mcts.add_dirichlet_noise(mcts.root);
                    sample
                })
                .collect();

            for (sample, _) in new_samples {
                if let Some(sample) = sample {
                    info!("{}", sample);
                    replay_buffer.push(sample);
                }
            }

            // info!("Replay Buffer size: {}", replay_buffer.buffer.len());
        }

        if replay_buffer.buffer.len() < 32768 {
            continue;
        }

        let total_batches = (training_config.steps_per_iter * mcts_config.num_simulations) as f64;
        let avg_illegal_prob = illegal_move_weight / total_batches;
        info!("illegal weight: {}\ntotal_batches: {}", illegal_move_weight, total_batches);

        let mut loss_val: f32 = 0.0;
        for epoch in 0..training_config.gradient_steps {
            info!("Training model: epoch {}", epoch);
            let (new_model, val) = train(model.clone(), &mut optimizer, &mut lr_scheduler, &training_config, &replay_buffer, device, avg_illegal_prob as f32, &mut rng);
            model = new_model;
            loss_val += val;
        }
        loss_val /= training_config.gradient_steps as f32;


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
            let snapshot_path = format!("{}/model-{}", artifact_dir, (iterations / 25) % 10);
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
    scheduler: &mut NoamLrScheduler,
    config: &TrainingConfig,
    games: &ReplayBuffer,
    device: &B::Device,
    avg_illegal_prob: f32,
    rng: &mut SmallRng,
) -> (ChessTransformer<B>, f32) {
    let datas: ChessBatch<B> = games.sample_batch(config.batch_size, rng, device);
    let lr = scheduler.step();

    let output = model.forward_classification(datas, avg_illegal_prob);
    let loss = output.loss;
    let grads = loss.backward();
    let grads = GradientsParams::from_grads(grads, &model);


    let loss_val = loss.clone().into_scalar().to_f32();
    (optimizer.step(lr, model.clone(), grads), loss_val)
}

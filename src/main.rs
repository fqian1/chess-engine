#![recursion_limit = "256"]

use std::io::{self, Write};
use std::path::PathBuf;

use burn::backend::Autodiff;
use burn::lr_scheduler::noam::NoamLrSchedulerConfig;
use burn::module::Module;
use burn::optim::AdamWConfig;
use burn::record::{FullPrecisionSettings, NamedMpkFileRecorder, Recorder};
use chess_engine::model::ChessTransformerConfig;
use chess_engine::*;
use clap::Parser;
use env_logger::Builder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 256)]
    batch_size: usize,
    #[arg(short, long)]
    legal: bool,
    #[arg(short, long)]
    masked: bool,
    #[arg(short, long, default_value_t = 1.25)]
    c_puct: f32,
    #[arg(short, long, default_value_t = 64)]
    gradient_steps: usize,
    #[arg(short, long, default_value_t = 1234)]
    seed: u64,
    #[arg(short, long, default_value_t = 256)]
    num_simulations: usize,
    #[arg(short, long, default_value_t = 64)]
    iter_count: usize,
    #[arg(short, long, default_value_t = 1.0)]
    temperature: f32,
    #[arg(short, long, value_name = "DIR")]
    path: PathBuf,
}

pub struct TrainingMetrics {
    pub epoch: u32,
    pub illegal_moves_generated: u32,
    pub total_moves_generated: u32,
    pub average_loss: u32,
    pub average_game_length: u32,
    pub unique_positions_seen: f32,
}

pub fn display_moves(moves: &Vec<ChessMove>) {
    for mv in moves {
        println!("{}", mv.to_uci());
    }
}

fn main() {
    Builder::new().filter_level(log::LevelFilter::Info).init();

    let args = Args::parse();

    let mut path = args.path.clone();
    if path.starts_with("~") {
        if let Ok(home) = std::env::var("HOME") {
            let path_str = path.to_str().unwrap_or("");
            path = PathBuf::from(path_str.replacen('~', &home, 1));
        }
    }
    let artifact_dir = path; // Use the expanded path

    #[cfg(feature = "cuda")]
    pub type MyInferenceBackend = burn::backend::Cuda<f32, i32>;

    #[cfg(not(feature = "cuda"))]
    pub type MyInferenceBackend = burn::backend::Wgpu<f32, i32>;

    type MyAutodiffBackend = Autodiff<MyInferenceBackend>;

    let artifact_dir_str = artifact_dir.to_str().unwrap_or("tmp/stats/");

    let device = Default::default();

    let mcts_config = MctsConfig { num_simulations: args.num_simulations, c_puct: args.c_puct, temperature: args.temperature, legal: args.legal };

    let size = 8;
    let n_heads = size;
    let n_layers = size;
    let d_model = n_heads * 64;
    let d_ff = 4 * d_model;

    let model_config = ChessTransformerConfig::new(d_model, n_heads, d_ff, n_layers);
    let optimizer_config = AdamWConfig::new().with_beta_1(0.9).with_beta_2(0.99).with_weight_decay(1e-4);
    let scheduler_config = NoamLrSchedulerConfig::new(0.01).with_model_size(512);

    let training_config = TrainingConfig {
        model: model_config,
        masked: args.masked,
        legal: args.legal,
        scheduler: scheduler_config,
        optimizer: optimizer_config,
        gradient_steps: args.gradient_steps,
        steps_per_iter: args.iter_count,
        batch_size: args.batch_size,
        num_workers: 8,
        seed: args.seed,
    };

    loop {
        println!("What do you want to do?\n1 - Train a model!\n2 - Inference");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        input = input.trim().to_lowercase();
        match input.as_str() {
            "1" => {
                println!("Using config: \n{:?}\n{:?}", training_config, mcts_config);
                println!("artifact dir: {:?}", artifact_dir_str);
                println!("backend: {}", std::any::type_name::<MyInferenceBackend>());

                play::<MyAutodiffBackend>(&artifact_dir_str, &mcts_config, &training_config, &device);
            }
            "2" => {
                println!("Enter fen string: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let game = ChessGame::from_fen(&input).unwrap_or_else(|_| {
                    println!("Failed to parse fen, creating default game");
                    ChessGame::default()
                });
                let _mcts = [Mcts::from_game(&game, 1000, mcts_config)];

                println!("printing artifact dir: {:?}", artifact_dir.clone());

                let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::default();
                let record = recorder.load(artifact_dir.clone(), &device).expect("Failed to load .mpk model record");

                let model: ChessTransformer<MyInferenceBackend> = training_config.model.init(&device);
                let model = model.load_record(record);

                let mut training_config = training_config.clone();
                training_config.batch_size = 1;

                let inputs = vec![NetworkInputs::from_position(&game.position, None)];

                let out = model_make_outputs(model.clone(), &inputs, &training_config, None, &device);

                let sq = out[0].as_squares().into_iter().max_by(|&a, &b| a.1.total_cmp(&b.1));

                let sq = sq.unwrap().0;

                let inputs = vec![NetworkInputs::from_position(&game.position, Some(&sq))];

                let out = model_make_outputs(model.clone(), &inputs, &training_config, None, &device);

                let sq2 = out[0].as_squares().into_iter().max_by(|&a, &b| a.1.total_cmp(&b.1));
                let sq2 = sq2.unwrap().0;

                let mov = ChessMove::new(sq, sq2, None);

                // this is just no mcts raw guess, doesnt handle promotions either
                print!("\nI picked: {}\n", mov.to_uci());
            }
            _ => println!("Invalid - select {{1|2}}"),
        }
    }

    // loop {
    //     ChessGame::fen_to_ascii(&game.to_fen());
    //     println!("{:?}'s turn.", game.position.side_to_move);
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
    //         Ok(input) => game.position.make_move(&input),
    //         Err(e) => println!("{e}"),
    //     }
    // }
}

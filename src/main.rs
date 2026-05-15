/*
Copyright (C) [2016] [Francois Qian]

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#![recursion_limit = "256"]
use crate::{ChessGame, ChessTransformer, Mcts, MctsConfig, expand_batch};
use burn::{lr_scheduler::noam::NoamLrSchedulerConfig, module::Module, optim::AdamWConfig};
use log::info;
use std::io::{self, Write};
use std::path::PathBuf;

use burn::backend::Autodiff;
use burn::record::{FullPrecisionSettings, NamedMpkFileRecorder, Recorder};
use chess_engine::model::ChessTransformerConfig;
use chess_engine::*;
use clap::Parser;

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
    #[arg(short, long)]
    annealing: bool,
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
    env_logger::init();

    let args = Args::parse();

    let mut path = args.path.clone();
    if path.starts_with("~")
        && let Ok(home) = std::env::var("HOME")
    {
        let path_str = path.to_str().unwrap_or("");
        path = PathBuf::from(path_str.replacen('~', &home, 1));
    }
    let artifact_dir = path;

    #[cfg(feature = "cuda")]
    pub type MyInferenceBackend = burn::backend::Cuda<f32, i32>;

    #[cfg(not(feature = "cuda"))]
    pub type MyInferenceBackend = burn::backend::Wgpu<f32, i32>;

    type MyAutodiffBackend = Autodiff<MyInferenceBackend>;

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
        annealing: args.annealing,
        scheduler: scheduler_config,
        optimizer: optimizer_config,
        gradient_steps: args.gradient_steps,
        steps_per_iter: args.iter_count,
        batch_size: args.batch_size,
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
                println!("artifact dir: {:?}", artifact_dir);
                println!("backend: {}", std::any::type_name::<MyInferenceBackend>());

                play::<MyAutodiffBackend>(&artifact_dir, &mcts_config, &training_config, &device);
            }
            "2" => {
                println!("Enter fen string: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let mut game = ChessGame::from_fen(&input).unwrap_or_else(|_| {
                    println!("Failed to parse fen, creating default game");
                    ChessGame::default()
                });

                let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::default();
                let record = recorder.load(artifact_dir.clone(), &device).expect("Failed to load .mpk model record");
                let model: ChessTransformer<MyInferenceBackend> = training_config.model.init(&device);
                let model = model.load_record(record);

                let mut inf_config = training_config.clone();
                inf_config.batch_size = 1;
                inf_config.masked = true;
                inf_config.legal = mcts_config.legal;

                let mut mcts = Mcts::from_game(&game, 65536, mcts_config, 1234);

                println!("Running first pass (choose from square)...");
                for _ in 0..mcts_config.num_simulations {
                    mcts.traverse_get_terminal();
                    let mcts_ref = std::slice::from_mut(&mut mcts);
                    expand_batch(mcts_ref, model.clone(), &inf_config, &device);
                }

                let root_node = &mcts.node_arena.buffer[mcts.root];
                let (start, end) = root_node.get_data().child_edge_range.expect("Root not expanded");
                let best_from_edge = mcts.edge_arena.buffer[start..end].iter().max_by_key(|e| e.visits).expect("No edges");
                let from_sq = best_from_edge.square;
                let piece_move_node_idx = best_from_edge.child_node_idx.expect("Edge not expanded");

                mcts.root = piece_move_node_idx;

                println!("Running second pass (choose to square)...");
                for _ in 0..mcts_config.num_simulations {
                    mcts.traverse_get_terminal();
                    let mcts_ref = std::slice::from_mut(&mut mcts);
                    expand_batch(mcts_ref, model.clone(), &inf_config, &device);
                }

                let move_root = &mcts.node_arena.buffer[mcts.root];
                let (start2, end2) = move_root.get_data().child_edge_range.expect("Move root not expanded");
                let best_to_edge = mcts.edge_arena.buffer[start2..end2].iter().max_by_key(|e| e.visits).expect("No to-edges");
                let to_sq = best_to_edge.square;
                let promotion = best_to_edge.promotion_piece;

                let mov = ChessMove::new(from_sq, to_sq, promotion);
                println!("\nI picked: {}", mov.to_uci());
                game.make_move(&mov);
                println!("{}", game.position.to_fen());
            }
            _ => println!("Invalid - select {{1|2}}"),
        }
    }
}

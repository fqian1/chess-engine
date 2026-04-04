#![recursion_limit = "256"]

use burn::backend::{Autodiff, Wgpu};
use burn::optim::AdamConfig;
use chess_engine::model::ChessTransformerConfig;
use chess_engine::*;
use env_logger::Builder;

pub struct TrainingMetrics {
    pub epoch: u32,
    pub illegal_moves_generated: u32,
    pub total_moves_generated: u32,
    pub average_loss: u32,
    pub average_game_length: u32,
    pub unique_positions_seen: u32,
}

pub fn display_moves(moves: &Vec<ChessMove>) {
    for mv in moves {
        println!("{}", mv.to_uci());
    }
}

fn main() {
    Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    type MyInferenceBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyInferenceBackend>;

    let artifact_dir = "tmp/stats";

    let device = burn::backend::wgpu::WgpuDevice::default();

    let mcts_config = MctsConfig { num_simulations: 100, c_puct: 1.25, temperature: 0.01, legal: true };

    let size = 8;
    let n_heads = size;
    let n_layers = size;
    let d_model = n_heads * 64;
    let d_ff = 4 * d_model;

    let model_config = ChessTransformerConfig::new(d_model, n_heads, d_ff, n_layers);
    let optimizer_config = AdamConfig::new();

    let training_config = TrainingConfig {
        model: model_config,
        masked: true,
        legal: true,
        optimizer: optimizer_config,
        num_epochs: 100,
        batch_size: 100,
        num_workers: 8,
        seed: 1234,
        learning_rate: 0.001,
    };

    play::<MyAutodiffBackend>(&artifact_dir, &mcts_config, &training_config, &device);

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

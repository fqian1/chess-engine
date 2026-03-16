use burn::backend::{Autodiff, Wgpu};
use burn::module::AutodiffModule;
use chess_engine::data::TrainingSample;
use chess_engine::*;
use std::io;
use std::io::Write;

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
    type MyInferenceBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyInferenceBackend>;

    let device = burn::backend::Wgpu::default();
    let artifact_dir = "tmp/stats";
    let mut optimizer = config.optimizer.init();

    // let size = model_size * 4;
    // let n_heads = size;
    // let n_layers = size;
    // let d_model = n_heads * 64;
    // let d_ff = 4 * d_model;

    let mut training_data: Vec<TrainingSample> = Vec::with_capacity(30000);
    let mut chessgames: Vec<ChessGame> = vec![ChessGame::default(); 1024];
    chessgames.iter_mut().for_each(|game| {
        game.zobrist_hash = game.calculate_hash();
        game.rule_set = chess_game::RuleSet::Legal;
    });

    loop {
        let model = config.model.init::<B>(&device, true);
        chessgames.iter_mut().for_each(|game| game.outcome = game.check_game_state());
    }

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
}

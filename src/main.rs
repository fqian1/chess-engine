use chess_engine::*;
use std::io;
use std::io::Write;
use burn::backend::Wgpu;

pub struct TrainingMetrics {
    pub epoch: u32,
    pub illegal_moves_generate: u32,
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
    // type MyBackend = Wgpu<f32, i32>;

    // let size = model_size * 4;
    // let n_heads = size;
    // let n_layers = size;
    // let d_model = n_heads * 64;
    // let d_ff = 4 * d_model;
    //

    let mut game = ChessGame::default();
    game.zobrist_hash = game.calculate_hash();

    let mut chessgames: Vec<ChessGame> = vec![ChessGame::default(); 80];
    chessgames.iter_mut().for_each(|game| {
        game.zobrist_hash = game.calculate_hash()
    });

    let moves: Vec<ChessMove> = game.generate_pseudolegal().into_iter().collect();
    display_moves(&moves);
    println!("{}", game.chessboard.display_ascii());
    // loop
    // 1. convert to transformer representation
    // 2. check game state
    // 3. run inference, collect from squares
    // 4. run inference, collect to squares
    // 5. run inference, collect promotion square
    // 6. generate ~100 candidate moves, order by from score, to score
    // 7. generate moves
    // 8. check move validity
    // 9.
    // 10.

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

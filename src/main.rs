use chess_engine::*;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fmt;
use std::io;
use std::io::Write;

fn main() {
    let mut game = ChessGame::default();

    loop {
        ChessGame::fen_to_ascii(&game.to_fen());
        println!("{:?}'s turn.", game.side_to_move);

        print!("Enter move (e.g., e2e4): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input == "quit" || input == "exit" {
            break;
        }

        if input == "debug" {
            println!("{game:?}");
        }

        let input = game.uci_to_move(&input);
        match input {
            Ok(input) => game.make_move(&input),
            Err(e) => println!("{e}"),
        }
    }
}

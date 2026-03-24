#![recursion_limit = "256"]

#![feature(const_trait_impl)]
pub mod bitboard;
pub mod castling;
pub mod chess_board;
pub mod chess_game;
pub mod chess_move;
pub mod chess_piece;
pub mod chess_square;
pub mod zobrist;
pub mod model;
pub mod data;
pub mod engine;
pub mod chess_position;
pub mod mcts;

pub use burn;
pub use bitboard::Bitboard;
pub use castling::CastlingRights;
pub use chess_board::ChessBoard;
pub use chess_game::ChessGame;
pub use chess_move::ChessMove;
pub use chess_piece::{ChessPiece, Color, PieceType};
pub use chess_square::ChessSquare;
pub use engine::*;
pub use zobrist::ZobristKeys;
pub use model::ChessTransformer;
pub use data::*;
pub use chess_position::ChessPosition;
pub use mcts::*;

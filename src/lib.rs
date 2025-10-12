#![feature(const_trait_impl)]
pub mod bitboard;
pub mod castling;
pub mod chess_board;
pub mod chess_game;
pub mod chess_move;
pub mod chess_piece;
pub mod chess_square;

#[doc(inline)]
pub use bitboard::Bitboard;
#[doc(inline)]
pub use castling::CastlingRights;
#[doc(inline)]
pub use chess_board::ChessBoard;
#[doc(inline)]
pub use chess_game::ChessGame;
#[doc(inline)]
pub use chess_move::ChessMove;
#[doc(inline)]
pub use chess_piece::{ChessPiece, Color, PieceType};
#[doc(inline)]
pub use chess_square::ChessSquare;

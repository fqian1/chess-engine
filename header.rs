pub mod chess_engine
pub use chess_engine::burn
pub mod chess_engine::bitboard
pub struct chess_engine::bitboard::Bitboard(pub u64)
impl chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::ALL: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::ALL_PIECES: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_BISHOPS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_KING: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_KNIGHTS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_OCCUPANCY: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_PAWNS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_QUEENS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_ROOKS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::EMPTY: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_BISHOPS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_KING: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_KNIGHTS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_OCCUPANCY: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_PAWNS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_QUEENS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_ROOKS: chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::clear(&mut self, square: chess_engine::chess_square::ChessSquare)
pub fn chess_engine::bitboard::Bitboard::count(&self) -> u32
pub fn chess_engine::bitboard::Bitboard::flip(&mut self)
pub fn chess_engine::bitboard::Bitboard::flipped(&self) -> chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::from_square(square: chess_engine::chess_square::ChessSquare) -> Self
pub fn chess_engine::bitboard::Bitboard::is_empty(&self) -> bool
pub fn chess_engine::bitboard::Bitboard::is_set(&self, square: chess_engine::chess_square::ChessSquare) -> bool
pub fn chess_engine::bitboard::Bitboard::lsb_square(&self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub fn chess_engine::bitboard::Bitboard::msb_square(&self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub fn chess_engine::bitboard::Bitboard::pop_lsb(&mut self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub fn chess_engine::bitboard::Bitboard::pop_msb(&mut self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub const fn chess_engine::bitboard::Bitboard::set(&mut self, square: chess_engine::chess_square::ChessSquare)
pub fn chess_engine::bitboard::Bitboard::shift_east(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_north(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_north_east(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_north_west(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_south(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_south_east(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_south_west(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_west(self) -> Self
pub fn chess_engine::bitboard::Bitboard::to_bool(&self) -> [bool; 64]
pub fn chess_engine::bitboard::Bitboard::toggle(&mut self, square: chess_engine::chess_square::ChessSquare)
pub fn chess_engine::bitboard::Bitboard::write_to_slice(&self, slice: &mut [f32])
impl core::fmt::Display for chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::ops::bit::BitAnd for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitand(self, rhs: Self) -> Self::Output
impl core::ops::bit::BitAnd<&chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitand(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitAnd<&chess_engine::bitboard::Bitboard> for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitand(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitAnd<chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitand(self, rhs: chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitAndAssign for chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitand_assign(&mut self, rhs: Self)
impl core::ops::bit::BitOr for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitor(self, rhs: Self) -> Self::Output
impl core::ops::bit::BitOr<&chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitor(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitOr<&chess_engine::bitboard::Bitboard> for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitor(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitOr<chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitor(self, rhs: chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitOrAssign for chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitor_assign(&mut self, rhs: Self)
impl core::ops::bit::BitXor for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitxor(self, rhs: Self) -> Self::Output
impl core::ops::bit::BitXor<&chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitxor(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitXor<&chess_engine::bitboard::Bitboard> for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitxor(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitXor<chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitxor(self, rhs: chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitXorAssign for chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitxor_assign(&mut self, rhs: Self)
impl core::ops::bit::Not for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::not(self) -> Self::Output
pub mod chess_engine::castling
pub struct chess_engine::castling::CastlingRights(pub u8)
impl chess_engine::castling::CastlingRights
pub const chess_engine::castling::CastlingRights::BLACK_KINGSIDE: chess_engine::castling::CastlingRights
pub const chess_engine::castling::CastlingRights::BLACK_QUEENSIDE: chess_engine::castling::CastlingRights
pub const chess_engine::castling::CastlingRights::WHITE_KINGSIDE: chess_engine::castling::CastlingRights
pub const chess_engine::castling::CastlingRights::WHITE_QUEENSIDE: chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::empty() -> Self
pub fn chess_engine::castling::CastlingRights::flip_perspective(&self) -> chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::from_fen(fen_part: &str) -> Self
pub fn chess_engine::castling::CastlingRights::has(&self, right: chess_engine::castling::CastlingRights) -> bool
pub fn chess_engine::castling::CastlingRights::new() -> Self
pub fn chess_engine::castling::CastlingRights::remove(&mut self, rights_to_remove: chess_engine::castling::CastlingRights)
pub fn chess_engine::castling::CastlingRights::to_fen(&self) -> alloc::string::String
impl core::ops::bit::BitAndAssign for chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::bitand_assign(&mut self, rhs: Self)
impl core::ops::bit::BitOr for chess_engine::castling::CastlingRights
pub type chess_engine::castling::CastlingRights::Output = chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::bitor(self, rhs: Self) -> Self::Output
impl core::ops::bit::BitOr<chess_engine::castling::CastlingRights> for u8
pub type u8::Output = chess_engine::castling::CastlingRights
pub fn u8::bitor(self, rhs: chess_engine::castling::CastlingRights) -> Self::Output
impl core::ops::bit::BitOr<u8> for chess_engine::castling::CastlingRights
pub type chess_engine::castling::CastlingRights::Output = chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::bitor(self, rhs: u8) -> Self::Output
impl core::ops::bit::BitOrAssign for chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::bitor_assign(&mut self, rhs: Self)
impl core::ops::bit::BitXor for chess_engine::castling::CastlingRights
pub type chess_engine::castling::CastlingRights::Output = chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::bitxor(self, rhs: Self) -> Self::Output
impl core::ops::bit::Not for chess_engine::castling::CastlingRights
pub type chess_engine::castling::CastlingRights::Output = chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::not(self) -> Self::Output
pub mod chess_engine::chess_board
pub struct chess_engine::chess_board::ChessBoard
pub chess_engine::chess_board::ChessBoard::all_pieces: chess_engine::bitboard::Bitboard
pub chess_engine::chess_board::ChessBoard::black_occupancy: chess_engine::bitboard::Bitboard
pub chess_engine::chess_board::ChessBoard::pieces: [[chess_engine::bitboard::Bitboard; 6]; 2]
pub chess_engine::chess_board::ChessBoard::white_occupancy: chess_engine::bitboard::Bitboard
impl chess_engine::chess_board::ChessBoard
pub const chess_engine::chess_board::ChessBoard::BETWEEN: [[core::option::Option<chess_engine::bitboard::Bitboard>; 64]; 64]
pub const chess_engine::chess_board::ChessBoard::BISHOP_ATTACKS: [[chess_engine::bitboard::Bitboard; 4]; 64]
pub const chess_engine::chess_board::ChessBoard::BLACK_SQUARES: chess_engine::bitboard::Bitboard
pub const chess_engine::chess_board::ChessBoard::KING_ATTACKS: [chess_engine::bitboard::Bitboard; 64]
pub const chess_engine::chess_board::ChessBoard::KNIGHT_ATTACKS: [chess_engine::bitboard::Bitboard; 64]
pub const chess_engine::chess_board::ChessBoard::PAWN_ATTACKS_BLACK: [chess_engine::bitboard::Bitboard; 64]
pub const chess_engine::chess_board::ChessBoard::PAWN_ATTACKS_WHITE: [chess_engine::bitboard::Bitboard; 64]
pub const chess_engine::chess_board::ChessBoard::ROOK_ATTACKS: [[chess_engine::bitboard::Bitboard; 4]; 64]
pub const chess_engine::chess_board::ChessBoard::WHITE_SQUARES: chess_engine::bitboard::Bitboard
pub fn chess_engine::chess_board::ChessBoard::add_piece(&mut self, piece: chess_engine::chess_piece::ChessPiece, square: chess_engine::chess_square::ChessSquare)
pub fn chess_engine::chess_board::ChessBoard::apply_move(&mut self, mov: &chess_engine::chess_move::ChessMove, side_to_move: chess_engine::chess_piece::Color, en_passant_sq: core::option::Option<chess_engine::chess_square::ChessSquare>)
pub fn chess_engine::chess_board::ChessBoard::display_ascii(&self) -> alloc::string::String
pub fn chess_engine::chess_board::ChessBoard::empty() -> Self
pub fn chess_engine::chess_board::ChessBoard::flip_board(&self) -> Self
pub const fn chess_engine::chess_board::ChessBoard::generate_rook_direction_masks() -> [[chess_engine::bitboard::Bitboard; 4]; 64]
pub fn chess_engine::chess_board::ChessBoard::get_piece_at(&self, square: chess_engine::chess_square::ChessSquare) -> core::option::Option<chess_engine::chess_piece::ChessPiece>
pub fn chess_engine::chess_board::ChessBoard::get_piece_bitboard(&self, color: chess_engine::chess_piece::Color, piece_type: chess_engine::chess_piece::PieceType) -> chess_engine::bitboard::Bitboard
pub fn chess_engine::chess_board::ChessBoard::is_square_attacked(&self, sq: chess_engine::chess_square::ChessSquare, attacker_color: chess_engine::chess_piece::Color) -> bool
pub fn chess_engine::chess_board::ChessBoard::move_piece(&mut self, from_sq: chess_engine::chess_square::ChessSquare, to_sq: chess_engine::chess_square::ChessSquare, piece: chess_engine::chess_piece::ChessPiece)
pub fn chess_engine::chess_board::ChessBoard::new() -> Self
pub fn chess_engine::chess_board::ChessBoard::remove_piece(&mut self, piece: chess_engine::chess_piece::ChessPiece, square: chess_engine::chess_square::ChessSquare)
pub mod chess_engine::chess_game
pub enum chess_engine::chess_game::Outcome
pub chess_engine::chess_game::Outcome::Finished(core::option::Option<chess_engine::chess_piece::Color>)
pub chess_engine::chess_game::Outcome::Unfinished
impl chess_engine::chess_game::Outcome
pub fn chess_engine::chess_game::Outcome::to_f32(&self) -> core::option::Option<[f32; 3]>
pub enum chess_engine::chess_game::RuleSet
pub chess_engine::chess_game::RuleSet::Legal
pub chess_engine::chess_game::RuleSet::PseudoLegal
impl chess_engine::chess_game::RuleSet
pub fn chess_engine::chess_game::RuleSet::is_legal(&self) -> bool
pub struct chess_engine::chess_game::ChessGame
pub chess_engine::chess_game::ChessGame::fullmove_counter: u32
pub chess_engine::chess_game::ChessGame::game_history: alloc::vec::Vec<chess_engine::chess_position::ChessPosition>
pub chess_engine::chess_game::ChessGame::outcome: chess_engine::chess_game::Outcome
pub chess_engine::chess_game::ChessGame::position: chess_engine::chess_position::ChessPosition
pub chess_engine::chess_game::ChessGame::rule_set: chess_engine::chess_game::RuleSet
impl chess_engine::chess_game::ChessGame
pub fn chess_engine::chess_game::ChessGame::check_game_state(&self) -> chess_engine::chess_game::Outcome
pub fn chess_engine::chess_game::ChessGame::fen_to_ascii(fen: &str)
pub fn chess_engine::chess_game::ChessGame::from_fen(fen: &str) -> Self
pub fn chess_engine::chess_game::ChessGame::make_move(&mut self, mov: &chess_engine::chess_move::ChessMove)
pub fn chess_engine::chess_game::ChessGame::to_fen(&self) -> alloc::string::String
pub fn chess_engine::chess_game::ChessGame::uci_to_move(&self, input: &str) -> core::result::Result<chess_engine::chess_move::ChessMove, &str>
pub fn chess_engine::chess_game::ChessGame::unmake_move(&mut self)
impl core::default::Default for chess_engine::chess_game::ChessGame
pub fn chess_engine::chess_game::ChessGame::default() -> Self
pub mod chess_engine::chess_move
pub struct chess_engine::chess_move::ChessMove
pub chess_engine::chess_move::ChessMove::from: chess_engine::chess_square::ChessSquare
pub chess_engine::chess_move::ChessMove::promotion: core::option::Option<chess_engine::chess_piece::PieceType>
pub chess_engine::chess_move::ChessMove::to: chess_engine::chess_square::ChessSquare
impl chess_engine::chess_move::ChessMove
pub fn chess_engine::chess_move::ChessMove::from_uci(uci: &str) -> core::result::Result<Self, &'static str>
pub fn chess_engine::chess_move::ChessMove::new(from: chess_engine::chess_square::ChessSquare, to: chess_engine::chess_square::ChessSquare, promotion: core::option::Option<chess_engine::chess_piece::PieceType>) -> Self
pub fn chess_engine::chess_move::ChessMove::to_uci(&self) -> alloc::string::String
pub mod chess_engine::chess_piece
#[repr(usize)] pub enum chess_engine::chess_piece::Color
pub chess_engine::chess_piece::Color::Black = 1
pub chess_engine::chess_piece::Color::White = 0
impl chess_engine::chess_piece::Color
pub fn chess_engine::chess_piece::Color::from_char(c: char) -> core::option::Option<Self>
pub fn chess_engine::chess_piece::Color::opposite(&self) -> Self
pub fn chess_engine::chess_piece::Color::to_char(&self) -> char
#[repr(usize)] pub enum chess_engine::chess_piece::PieceType
pub chess_engine::chess_piece::PieceType::Bishop = 2
pub chess_engine::chess_piece::PieceType::King = 5
pub chess_engine::chess_piece::PieceType::Knight = 1
pub chess_engine::chess_piece::PieceType::Pawn = 0
pub chess_engine::chess_piece::PieceType::Queen = 4
pub chess_engine::chess_piece::PieceType::Rook = 3
impl chess_engine::chess_piece::PieceType
pub fn chess_engine::chess_piece::PieceType::from_char(c: char) -> core::option::Option<Self>
pub fn chess_engine::chess_piece::PieceType::from_idx(idx: usize) -> core::option::Option<chess_engine::chess_piece::PieceType>
pub fn chess_engine::chess_piece::PieceType::to_char(&self, color: chess_engine::chess_piece::Color) -> char
pub struct chess_engine::chess_piece::ChessPiece
pub chess_engine::chess_piece::ChessPiece::color: chess_engine::chess_piece::Color
pub chess_engine::chess_piece::ChessPiece::piece_type: chess_engine::chess_piece::PieceType
impl chess_engine::chess_piece::ChessPiece
pub fn chess_engine::chess_piece::ChessPiece::new(color: chess_engine::chess_piece::Color, piece_type: chess_engine::chess_piece::PieceType) -> Self
pub mod chess_engine::chess_position
pub struct chess_engine::chess_position::ChessPosition
pub chess_engine::chess_position::ChessPosition::castling_rights: chess_engine::castling::CastlingRights
pub chess_engine::chess_position::ChessPosition::chessboard: chess_engine::chess_board::ChessBoard
pub chess_engine::chess_position::ChessPosition::en_passant: core::option::Option<chess_engine::chess_square::ChessSquare>
pub chess_engine::chess_position::ChessPosition::halfmove_clock: u32
pub chess_engine::chess_position::ChessPosition::pseudolegal_moves: alloc::vec::Vec<chess_engine::chess_move::ChessMove>
pub chess_engine::chess_position::ChessPosition::side_to_move: chess_engine::chess_piece::Color
pub chess_engine::chess_position::ChessPosition::zobrist_hash: u64
impl chess_engine::chess_position::ChessPosition
pub fn chess_engine::chess_position::ChessPosition::calculate_hash(&self) -> u64
pub fn chess_engine::chess_position::ChessPosition::check_game_state(&self, rule_set: chess_engine::chess_game::RuleSet) -> chess_engine::chess_game::Outcome
pub fn chess_engine::chess_position::ChessPosition::generate_pseudolegal(&self) -> alloc::vec::Vec<chess_engine::chess_move::ChessMove>
pub fn chess_engine::chess_position::ChessPosition::get_squares(&self) -> (alloc::vec::Vec<chess_engine::chess_square::ChessSquare>, alloc::vec::Vec<chess_engine::chess_square::ChessSquare>)
pub fn chess_engine::chess_position::ChessPosition::is_geometrically_valid(&self, mov: &chess_engine::chess_move::ChessMove) -> bool
pub fn chess_engine::chess_position::ChessPosition::is_legal(&self, mov: &chess_engine::chess_move::ChessMove) -> bool
pub fn chess_engine::chess_position::ChessPosition::make_move(&mut self, mov: &chess_engine::chess_move::ChessMove)
pub mod chess_engine::chess_square
pub struct chess_engine::chess_square::ChessSquare(pub u8)
impl chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H8: chess_engine::chess_square::ChessSquare
pub fn chess_engine::chess_square::ChessSquare::bitboard(self) -> chess_engine::bitboard::Bitboard
pub fn chess_engine::chess_square::ChessSquare::colour(self) -> chess_engine::chess_piece::Color
pub fn chess_engine::chess_square::ChessSquare::file(&self) -> u8
pub const fn chess_engine::chess_square::ChessSquare::from_coords(file: u8, rank: u8) -> core::option::Option<Self>
pub fn chess_engine::chess_square::ChessSquare::from_name(name: &str) -> core::option::Option<Self>
pub fn chess_engine::chess_square::ChessSquare::index(&self) -> u8
pub fn chess_engine::chess_square::ChessSquare::is_valid(self) -> bool
pub fn chess_engine::chess_square::ChessSquare::name(&self) -> alloc::string::String
pub const fn chess_engine::chess_square::ChessSquare::new(index: u8) -> core::option::Option<Self>
pub fn chess_engine::chess_square::ChessSquare::rank(&self) -> u8
pub fn chess_engine::chess_square::ChessSquare::square_north(self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub fn chess_engine::chess_square::ChessSquare::square_opposite(&self) -> chess_engine::chess_square::ChessSquare
pub fn chess_engine::chess_square::ChessSquare::square_south(self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub fn chess_engine::chess_square::ChessSquare::to_name(&self) -> alloc::string::String
impl core::fmt::Display for chess_engine::chess_square::ChessSquare
pub fn chess_engine::chess_square::ChessSquare::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
pub mod chess_engine::data
pub struct chess_engine::data::ChessBatch<B: burn_backend::backend::base::Backend>
pub chess_engine::data::ChessBatch::boards: burn_tensor::tensor::api::base::Tensor<B, 3>
pub chess_engine::data::ChessBatch::metas: burn_tensor::tensor::api::base::Tensor<B, 2>
pub chess_engine::data::ChessBatch::policy_targets: burn_tensor::tensor::api::base::Tensor<B, 2>
pub chess_engine::data::ChessBatch::value_targets: burn_tensor::tensor::api::base::Tensor<B, 2>
impl<B: burn_backend::backend::base::Backend> burn_core::data::dataloader::batcher::Batcher<B, chess_engine::TrainingSample, chess_engine::ChessBatch<B>> for chess_engine::ChessBatcher
pub fn chess_engine::ChessBatcher::batch(&self, items: alloc::vec::Vec<chess_engine::TrainingSample>, device: &<B as burn_backend::backend::base::Backend>::Device) -> chess_engine::ChessBatch<B>
pub struct chess_engine::data::ChessBatcher
impl<B: burn_backend::backend::base::Backend> burn_core::data::dataloader::batcher::Batcher<B, chess_engine::TrainingSample, chess_engine::ChessBatch<B>> for chess_engine::ChessBatcher
pub fn chess_engine::ChessBatcher::batch(&self, items: alloc::vec::Vec<chess_engine::TrainingSample>, device: &<B as burn_backend::backend::base::Backend>::Device) -> chess_engine::ChessBatch<B>
pub struct chess_engine::data::NetworkInputs
pub chess_engine::data::NetworkInputs::boards: [f32; 896]
pub chess_engine::data::NetworkInputs::meta: [f32; 5]
impl chess_engine::NetworkInputs
pub fn chess_engine::NetworkInputs::from_position(position: &chess_engine::chess_position::ChessPosition, selected_sq: core::option::Option<chess_engine::chess_square::ChessSquare>) -> Self
pub fn chess_engine::NetworkInputs::new(position: &chess_engine::chess_position::ChessPosition) -> Self
impl core::default::Default for chess_engine::NetworkInputs
pub fn chess_engine::NetworkInputs::default() -> Self
pub struct chess_engine::data::NetworkLabels
pub chess_engine::data::NetworkLabels::policy: [f32; 64]
pub chess_engine::data::NetworkLabels::value: [f32; 3]
impl chess_engine::NetworkLabels
pub fn chess_engine::NetworkLabels::as_squares(&self) -> [(chess_engine::chess_square::ChessSquare, f32); 64]
impl core::default::Default for chess_engine::NetworkLabels
pub fn chess_engine::NetworkLabels::default() -> Self
pub struct chess_engine::data::ReplayBuffer
pub chess_engine::data::ReplayBuffer::buffer: alloc::vec::Vec<chess_engine::TrainingSample>
pub chess_engine::data::ReplayBuffer::capacity: usize
pub chess_engine::data::ReplayBuffer::pointer: usize
impl chess_engine::ReplayBuffer
pub fn chess_engine::ReplayBuffer::new(capacity: usize) -> Self
pub fn chess_engine::ReplayBuffer::push(&mut self, sample: &chess_engine::TrainingSample)
pub fn chess_engine::ReplayBuffer::sample_batch<B: burn_backend::backend::base::Backend>(&self, batch_size: usize, rng: &mut rand::rngs::small::SmallRng, device: &<B as burn_backend::backend::base::Backend>::Device) -> chess_engine::ChessBatch<B>
pub struct chess_engine::data::TrainingSample
pub chess_engine::data::TrainingSample::inputs: chess_engine::NetworkInputs
pub chess_engine::data::TrainingSample::targets: chess_engine::NetworkLabels
impl core::default::Default for chess_engine::TrainingSample
pub fn chess_engine::TrainingSample::default() -> Self
impl<B: burn_backend::backend::base::Backend> burn_core::data::dataloader::batcher::Batcher<B, chess_engine::TrainingSample, chess_engine::ChessBatch<B>> for chess_engine::ChessBatcher
pub fn chess_engine::ChessBatcher::batch(&self, items: alloc::vec::Vec<chess_engine::TrainingSample>, device: &<B as burn_backend::backend::base::Backend>::Device) -> chess_engine::ChessBatch<B>
pub mod chess_engine::engine
pub struct chess_engine::engine::TrainingConfig
pub chess_engine::engine::TrainingConfig::batch_size: usize
pub chess_engine::engine::TrainingConfig::learning_rate: f64
pub chess_engine::engine::TrainingConfig::legal: bool
pub chess_engine::engine::TrainingConfig::masked: bool
pub chess_engine::engine::TrainingConfig::model: chess_engine::model::ChessTransformerConfig
pub chess_engine::engine::TrainingConfig::num_epochs: usize
pub chess_engine::engine::TrainingConfig::num_workers: usize
pub chess_engine::engine::TrainingConfig::optimizer: burn_optim::optim::adam::AdamConfig
pub chess_engine::engine::TrainingConfig::seed: u64
impl chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::new(model: chess_engine::model::ChessTransformerConfig, masked: bool, legal: bool, optimizer: burn_optim::optim::adam::AdamConfig) -> Self
impl chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::with_batch_size(self, batch_size: usize) -> Self
pub fn chess_engine::TrainingConfig::with_learning_rate(self, learning_rate: f64) -> Self
pub fn chess_engine::TrainingConfig::with_num_epochs(self, num_epochs: usize) -> Self
pub fn chess_engine::TrainingConfig::with_num_workers(self, num_workers: usize) -> Self
pub fn chess_engine::TrainingConfig::with_seed(self, seed: u64) -> Self
impl burn_core::config::Config for chess_engine::TrainingConfig
impl core::clone::Clone for chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::clone(&self) -> Self
impl core::fmt::Display for chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl serde_core::ser::Serialize for chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::serialize<S>(&self, serializer: S) -> core::result::Result<<S as serde_core::ser::Serializer>::Ok, <S as serde_core::ser::Serializer>::Error> where S: serde_core::ser::Serializer
impl<'de> serde_core::de::Deserialize<'de> for chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::deserialize<D>(deserializer: D) -> core::result::Result<Self, <D as serde_core::de::Deserializer>::Error> where D: serde_core::de::Deserializer<'de>
pub fn chess_engine::engine::generate_self_play_data<B: burn_backend::backend::base::AutodiffBackend>(artifact_dir: &str, training_config: &chess_engine::TrainingConfig, device: &<B as burn_backend::backend::base::Backend>::Device) -> alloc::vec::Vec<chess_engine::TrainingSample>
pub fn chess_engine::engine::inputs_to_tensor<B: burn_backend::backend::base::Backend>(buffer: &alloc::vec::Vec<chess_engine::NetworkInputs>, device: &<B as burn_backend::backend::base::Backend>::Device) -> (burn_tensor::tensor::api::base::Tensor<B, 3>, burn_tensor::tensor::api::base::Tensor<B, 2>)
pub fn chess_engine::engine::model_make_outputs<B: burn_backend::backend::base::Backend>(model: chess_engine::model::ChessTransformer<B>, inputs: &alloc::vec::Vec<chess_engine::NetworkInputs>, config: &chess_engine::TrainingConfig, masks: core::option::Option<alloc::vec::Vec<bool>>, device: &<B as burn_backend::backend::base::Backend>::Device) -> alloc::vec::Vec<chess_engine::NetworkLabels>
pub fn chess_engine::engine::train<B: burn_backend::backend::base::AutodiffBackend>(model: chess_engine::model::ChessTransformer<B>, optimizer: &mut burn_optim::optim::simple::adaptor::OptimizerAdaptor<burn_optim::optim::adam::Adam, chess_engine::model::ChessTransformer<B>, B>, config: chess_engine::TrainingConfig, games: &chess_engine::ReplayBuffer, device: &<B as burn_backend::backend::base::Backend>::Device, rng: &mut rand::rngs::small::SmallRng)
pub mod chess_engine::mcts
pub struct chess_engine::mcts::MctsConfig
pub struct chess_engine::mcts::MctsEdge
pub chess_engine::mcts::MctsEdge::child: core::option::Option<usize>
pub chess_engine::mcts::MctsEdge::confidence: f32
pub chess_engine::mcts::MctsEdge::mean_value: [f32; 3]
pub chess_engine::mcts::MctsEdge::square: chess_engine::chess_square::ChessSquare
pub chess_engine::mcts::MctsEdge::total_value: [f32; 3]
pub chess_engine::mcts::MctsEdge::visits: u32
impl chess_engine::MctsEdge
pub fn chess_engine::MctsEdge::get_puct(&self, c_puct: f32, total_parent_visits: u32, side_to_move: chess_engine::chess_piece::Color) -> f32
pub fn chess_engine::MctsEdge::new(sq: chess_engine::chess_square::ChessSquare, confidence: f32) -> Self
pub fn chess_engine::MctsEdge::update(&mut self, value: [f32; 3])
pub struct chess_engine::mcts::MctsNodeAction<'a>
impl<'a> chess_engine::MctsNodeAction<'a>
pub fn chess_engine::MctsNodeAction<'a>::new(mcts_state: &'a chess_engine::MctsNodeState, selected_sq: chess_engine::chess_square::ChessSquare, parent: usize) -> Self
pub struct chess_engine::mcts::MctsNodeState
impl chess_engine::MctsNodeState
pub fn chess_engine::MctsNodeState::new(game: &chess_engine::chess_game::ChessGame, parent: core::option::Option<usize>) -> Self
pub fn chess_engine::mcts::expand<B: burn_backend::backend::base::Backend>(nodes: &mut alloc::vec::Vec<&impl Node>, model: chess_engine::model::ChessTransformer<B>, config: &chess_engine::TrainingConfig, device: &<B as burn_backend::backend::base::Backend>::Device)
pub fn chess_engine::mcts::run_mcts<B: burn_backend::backend::base::Backend>(games: &alloc::vec::Vec<chess_engine::chess_game::ChessGame>, model: chess_engine::model::ChessTransformer<B>, mcts_config: &chess_engine::MctsConfig, training_config: &chess_engine::TrainingConfig, device: &<B as burn_backend::backend::base::Backend>::Device)
pub mod chess_engine::model
pub struct chess_engine::model::ChessTransformer<B: burn_backend::backend::base::Backend>
impl<B: burn_backend::backend::base::Backend> chess_engine::model::ChessTransformer<B>
pub fn chess_engine::model::ChessTransformer<B>::forward(&self, board: burn_tensor::tensor::api::base::Tensor<B, 3>, meta: burn_tensor::tensor::api::base::Tensor<B, 2>) -> (burn_tensor::tensor::api::base::Tensor<B, 2>, burn_tensor::tensor::api::base::Tensor<B, 2>)
pub fn chess_engine::model::ChessTransformer<B>::forward_classification(&self, batch: chess_engine::ChessBatch<B>) -> burn_train::learner::classification::ClassificationOutput<B>
impl<B: burn_backend::backend::base::AutodiffBackend> burn_train::learner::train_val::TrainStep for chess_engine::model::ChessTransformer<B>
pub type chess_engine::model::ChessTransformer<B>::Input = chess_engine::ChessBatch<B>
pub type chess_engine::model::ChessTransformer<B>::Output = burn_train::learner::classification::ClassificationOutput<B>
pub fn chess_engine::model::ChessTransformer<B>::step(&self, batch: chess_engine::ChessBatch<B>) -> burn_train::learner::train_val::TrainOutput<burn_train::learner::classification::ClassificationOutput<B>>
impl<B: burn_backend::backend::base::Backend> burn_core::module::base::Module<B> for chess_engine::model::ChessTransformer<B>
pub type chess_engine::model::ChessTransformer<B>::Record = chess_engine::model::ChessTransformerRecord<B>
pub fn chess_engine::model::ChessTransformer<B>::collect_devices(&self, devices: burn_core::module::base::Devices<B>) -> burn_core::module::base::Devices<B>
pub fn chess_engine::model::ChessTransformer<B>::fork(self, device: &<B as burn_backend::backend::base::Backend>::Device) -> Self
pub fn chess_engine::model::ChessTransformer<B>::into_record(self) -> Self::Record
pub fn chess_engine::model::ChessTransformer<B>::load_record(self, record: Self::Record) -> Self
pub fn chess_engine::model::ChessTransformer<B>::map<Mapper: burn_core::module::base::ModuleMapper<B>>(self, mapper: &mut Mapper) -> Self
pub fn chess_engine::model::ChessTransformer<B>::num_params(&self) -> usize
pub fn chess_engine::model::ChessTransformer<B>::to_device(self, device: &<B as burn_backend::backend::base::Backend>::Device) -> Self
pub fn chess_engine::model::ChessTransformer<B>::visit<Visitor: burn_core::module::base::ModuleVisitor<B>>(&self, visitor: &mut Visitor)
impl<B: burn_backend::backend::base::Backend> burn_core::module::display::ModuleDisplay for chess_engine::model::ChessTransformer<B>
impl<B: burn_backend::backend::base::Backend> burn_core::module::display::ModuleDisplayDefault for chess_engine::model::ChessTransformer<B>
pub fn chess_engine::model::ChessTransformer<B>::content(&self, content: burn_core::module::display::Content) -> core::option::Option<burn_core::module::display::Content>
pub fn chess_engine::model::ChessTransformer<B>::num_params(&self) -> usize
impl<B: burn_backend::backend::base::Backend> burn_train::learner::train_val::InferenceStep for chess_engine::model::ChessTransformer<B>
pub type chess_engine::model::ChessTransformer<B>::Input = chess_engine::ChessBatch<B>
pub type chess_engine::model::ChessTransformer<B>::Output = burn_train::learner::classification::ClassificationOutput<B>
pub fn chess_engine::model::ChessTransformer<B>::step(&self, batch: chess_engine::ChessBatch<B>) -> burn_train::learner::classification::ClassificationOutput<B>
impl<B: burn_backend::backend::base::Backend> core::clone::Clone for chess_engine::model::ChessTransformer<B>
pub fn chess_engine::model::ChessTransformer<B>::clone(&self) -> Self
impl<B: burn_backend::backend::base::Backend> core::fmt::Display for chess_engine::model::ChessTransformer<B>
pub fn chess_engine::model::ChessTransformer<B>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<B> burn_core::module::base::AutodiffModule<B> for chess_engine::model::ChessTransformer<B> where B: burn_backend::backend::base::AutodiffBackend + burn_backend::backend::base::Backend, <B as burn_backend::backend::base::AutodiffBackend>::InnerBackend: burn_backend::backend::base::Backend
pub type chess_engine::model::ChessTransformer<B>::InnerModule = chess_engine::model::ChessTransformer<<B as burn_backend::backend::base::AutodiffBackend>::InnerBackend>
pub fn chess_engine::model::ChessTransformer<B>::from_inner(module: Self::InnerModule) -> Self
pub fn chess_engine::model::ChessTransformer<B>::valid(&self) -> Self::InnerModule
impl<B> burn_core::module::base::HasAutodiffModule<B> for chess_engine::model::ChessTransformer<<B as burn_backend::backend::base::AutodiffBackend>::InnerBackend> where B: burn_backend::backend::base::AutodiffBackend + burn_backend::backend::base::Backend, <B as burn_backend::backend::base::AutodiffBackend>::InnerBackend: burn_backend::backend::base::Backend
pub type chess_engine::model::ChessTransformer<<B as burn_backend::backend::base::AutodiffBackend>::InnerBackend>::TrainModule = chess_engine::model::ChessTransformer<B>
pub struct chess_engine::model::ChessTransformerConfig
impl chess_engine::model::ChessTransformerConfig
pub fn chess_engine::model::ChessTransformerConfig::init<B: burn_backend::backend::base::Backend>(&self, device: &<B as burn_backend::backend::base::Backend>::Device) -> chess_engine::model::ChessTransformer<B>
impl chess_engine::model::ChessTransformerConfig
pub fn chess_engine::model::ChessTransformerConfig::new(d_model: usize, n_heads: usize, d_ff: usize, n_layers: usize) -> Self
impl burn_core::config::Config for chess_engine::model::ChessTransformerConfig
impl core::clone::Clone for chess_engine::model::ChessTransformerConfig
pub fn chess_engine::model::ChessTransformerConfig::clone(&self) -> Self
impl core::fmt::Display for chess_engine::model::ChessTransformerConfig
pub fn chess_engine::model::ChessTransformerConfig::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl serde_core::ser::Serialize for chess_engine::model::ChessTransformerConfig
pub fn chess_engine::model::ChessTransformerConfig::serialize<S>(&self, serializer: S) -> core::result::Result<<S as serde_core::ser::Serializer>::Ok, <S as serde_core::ser::Serializer>::Error> where S: serde_core::ser::Serializer
impl<'de> serde_core::de::Deserialize<'de> for chess_engine::model::ChessTransformerConfig
pub fn chess_engine::model::ChessTransformerConfig::deserialize<D>(deserializer: D) -> core::result::Result<Self, <D as serde_core::de::Deserializer>::Error> where D: serde_core::de::Deserializer<'de>
pub struct chess_engine::model::ChessTransformerRecord<B: burn_backend::backend::base::Backend>
pub chess_engine::model::ChessTransformerRecord::coordinates: <burn_tensor::tensor::api::base::Tensor<B, 2, burn_backend::tensor::kind::Int> as burn_core::module::base::Module<B>>::Record
pub chess_engine::model::ChessTransformerRecord::d_model: <usize as burn_core::module::base::Module<B>>::Record
pub chess_engine::model::ChessTransformerRecord::meta_encoder: <burn_nn::modules::linear::Linear<B> as burn_core::module::base::Module<B>>::Record
pub chess_engine::model::ChessTransformerRecord::piece_encoder: <burn_nn::modules::linear::Linear<B> as burn_core::module::base::Module<B>>::Record
pub chess_engine::model::ChessTransformerRecord::policy: <burn_nn::modules::linear::Linear<B> as burn_core::module::base::Module<B>>::Record
pub chess_engine::model::ChessTransformerRecord::pos_embedding_x: <burn_nn::modules::embedding::Embedding<B> as burn_core::module::base::Module<B>>::Record
pub chess_engine::model::ChessTransformerRecord::pos_embedding_y: <burn_nn::modules::embedding::Embedding<B> as burn_core::module::base::Module<B>>::Record
pub chess_engine::model::ChessTransformerRecord::transformer: <burn_nn::modules::transformer::encoder::TransformerEncoder<B> as burn_core::module::base::Module<B>>::Record
pub chess_engine::model::ChessTransformerRecord::value: <burn_nn::modules::linear::Linear<B> as burn_core::module::base::Module<B>>::Record
impl<B: burn_backend::backend::base::Backend> burn_core::record::base::Record<B> for chess_engine::model::ChessTransformerRecord<B>
pub type chess_engine::model::ChessTransformerRecord<B>::Item<S: burn_core::record::settings::PrecisionSettings> = chess_engine::model::ChessTransformerRecordItem<B, S>
pub fn chess_engine::model::ChessTransformerRecord<B>::from_item<S: burn_core::record::settings::PrecisionSettings>(item: Self::Item, device: &<B as burn_backend::backend::base::Backend>::Device) -> Self
pub fn chess_engine::model::ChessTransformerRecord<B>::into_item<S: burn_core::record::settings::PrecisionSettings>(self) -> Self::Item
pub struct chess_engine::model::ChessTransformerRecordItem<B: burn_backend::backend::base::Backend, S: burn_core::record::settings::PrecisionSettings>
pub chess_engine::model::ChessTransformerRecordItem::coordinates: <<burn_tensor::tensor::api::base::Tensor<B, 2, burn_backend::tensor::kind::Int> as burn_core::module::base::Module<B>>::Record as burn_core::record::base::Record<B>>::Item
pub chess_engine::model::ChessTransformerRecordItem::d_model: <<usize as burn_core::module::base::Module<B>>::Record as burn_core::record::base::Record<B>>::Item
pub chess_engine::model::ChessTransformerRecordItem::meta_encoder: <<burn_nn::modules::linear::Linear<B> as burn_core::module::base::Module<B>>::Record as burn_core::record::base::Record<B>>::Item
pub chess_engine::model::ChessTransformerRecordItem::piece_encoder: <<burn_nn::modules::linear::Linear<B> as burn_core::module::base::Module<B>>::Record as burn_core::record::base::Record<B>>::Item
pub chess_engine::model::ChessTransformerRecordItem::policy: <<burn_nn::modules::linear::Linear<B> as burn_core::module::base::Module<B>>::Record as burn_core::record::base::Record<B>>::Item
pub chess_engine::model::ChessTransformerRecordItem::pos_embedding_x: <<burn_nn::modules::embedding::Embedding<B> as burn_core::module::base::Module<B>>::Record as burn_core::record::base::Record<B>>::Item
pub chess_engine::model::ChessTransformerRecordItem::pos_embedding_y: <<burn_nn::modules::embedding::Embedding<B> as burn_core::module::base::Module<B>>::Record as burn_core::record::base::Record<B>>::Item
pub chess_engine::model::ChessTransformerRecordItem::transformer: <<burn_nn::modules::transformer::encoder::TransformerEncoder<B> as burn_core::module::base::Module<B>>::Record as burn_core::record::base::Record<B>>::Item
pub chess_engine::model::ChessTransformerRecordItem::value: <<burn_nn::modules::linear::Linear<B> as burn_core::module::base::Module<B>>::Record as burn_core::record::base::Record<B>>::Item
impl<B: burn_backend::backend::base::Backend, S: burn_core::record::settings::PrecisionSettings> core::clone::Clone for chess_engine::model::ChessTransformerRecordItem<B, S>
pub fn chess_engine::model::ChessTransformerRecordItem<B, S>::clone(&self) -> Self
pub mod chess_engine::zobrist
pub struct chess_engine::zobrist::ZobristKeys
pub chess_engine::zobrist::ZobristKeys::castling: [u64; 16]
pub chess_engine::zobrist::ZobristKeys::en_passant: [u64; 8]
pub chess_engine::zobrist::ZobristKeys::pieces: [[[u64; 64]; 6]; 2]
pub chess_engine::zobrist::ZobristKeys::side_to_move: u64
impl chess_engine::zobrist::ZobristKeys
pub fn chess_engine::zobrist::ZobristKeys::get() -> &'static Self
pub fn chess_engine::zobrist::ZobristKeys::new() -> Self
#[repr(usize)] pub enum chess_engine::Color
pub chess_engine::Color::Black = 1
pub chess_engine::Color::White = 0
impl chess_engine::chess_piece::Color
pub fn chess_engine::chess_piece::Color::from_char(c: char) -> core::option::Option<Self>
pub fn chess_engine::chess_piece::Color::opposite(&self) -> Self
pub fn chess_engine::chess_piece::Color::to_char(&self) -> char
#[repr(usize)] pub enum chess_engine::PieceType
pub chess_engine::PieceType::Bishop = 2
pub chess_engine::PieceType::King = 5
pub chess_engine::PieceType::Knight = 1
pub chess_engine::PieceType::Pawn = 0
pub chess_engine::PieceType::Queen = 4
pub chess_engine::PieceType::Rook = 3
impl chess_engine::chess_piece::PieceType
pub fn chess_engine::chess_piece::PieceType::from_char(c: char) -> core::option::Option<Self>
pub fn chess_engine::chess_piece::PieceType::from_idx(idx: usize) -> core::option::Option<chess_engine::chess_piece::PieceType>
pub fn chess_engine::chess_piece::PieceType::to_char(&self, color: chess_engine::chess_piece::Color) -> char
pub struct chess_engine::Bitboard(pub u64)
impl chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::ALL: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::ALL_PIECES: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_BISHOPS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_KING: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_KNIGHTS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_OCCUPANCY: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_PAWNS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_QUEENS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::BLACK_ROOKS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::EMPTY: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_BISHOPS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_KING: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_KNIGHTS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_OCCUPANCY: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_PAWNS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_QUEENS: chess_engine::bitboard::Bitboard
pub const chess_engine::bitboard::Bitboard::WHITE_ROOKS: chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::clear(&mut self, square: chess_engine::chess_square::ChessSquare)
pub fn chess_engine::bitboard::Bitboard::count(&self) -> u32
pub fn chess_engine::bitboard::Bitboard::flip(&mut self)
pub fn chess_engine::bitboard::Bitboard::flipped(&self) -> chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::from_square(square: chess_engine::chess_square::ChessSquare) -> Self
pub fn chess_engine::bitboard::Bitboard::is_empty(&self) -> bool
pub fn chess_engine::bitboard::Bitboard::is_set(&self, square: chess_engine::chess_square::ChessSquare) -> bool
pub fn chess_engine::bitboard::Bitboard::lsb_square(&self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub fn chess_engine::bitboard::Bitboard::msb_square(&self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub fn chess_engine::bitboard::Bitboard::pop_lsb(&mut self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub fn chess_engine::bitboard::Bitboard::pop_msb(&mut self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub const fn chess_engine::bitboard::Bitboard::set(&mut self, square: chess_engine::chess_square::ChessSquare)
pub fn chess_engine::bitboard::Bitboard::shift_east(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_north(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_north_east(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_north_west(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_south(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_south_east(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_south_west(self) -> Self
pub fn chess_engine::bitboard::Bitboard::shift_west(self) -> Self
pub fn chess_engine::bitboard::Bitboard::to_bool(&self) -> [bool; 64]
pub fn chess_engine::bitboard::Bitboard::toggle(&mut self, square: chess_engine::chess_square::ChessSquare)
pub fn chess_engine::bitboard::Bitboard::write_to_slice(&self, slice: &mut [f32])
impl core::fmt::Display for chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::ops::bit::BitAnd for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitand(self, rhs: Self) -> Self::Output
impl core::ops::bit::BitAnd<&chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitand(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitAnd<&chess_engine::bitboard::Bitboard> for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitand(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitAnd<chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitand(self, rhs: chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitAndAssign for chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitand_assign(&mut self, rhs: Self)
impl core::ops::bit::BitOr for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitor(self, rhs: Self) -> Self::Output
impl core::ops::bit::BitOr<&chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitor(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitOr<&chess_engine::bitboard::Bitboard> for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitor(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitOr<chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitor(self, rhs: chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitOrAssign for chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitor_assign(&mut self, rhs: Self)
impl core::ops::bit::BitXor for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitxor(self, rhs: Self) -> Self::Output
impl core::ops::bit::BitXor<&chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitxor(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitXor<&chess_engine::bitboard::Bitboard> for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitxor(self, rhs: &chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitXor<chess_engine::bitboard::Bitboard> for &chess_engine::bitboard::Bitboard
pub type &chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn &chess_engine::bitboard::Bitboard::bitxor(self, rhs: chess_engine::bitboard::Bitboard) -> Self::Output
impl core::ops::bit::BitXorAssign for chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::bitxor_assign(&mut self, rhs: Self)
impl core::ops::bit::Not for chess_engine::bitboard::Bitboard
pub type chess_engine::bitboard::Bitboard::Output = chess_engine::bitboard::Bitboard
pub fn chess_engine::bitboard::Bitboard::not(self) -> Self::Output
pub struct chess_engine::CastlingRights(pub u8)
impl chess_engine::castling::CastlingRights
pub const chess_engine::castling::CastlingRights::BLACK_KINGSIDE: chess_engine::castling::CastlingRights
pub const chess_engine::castling::CastlingRights::BLACK_QUEENSIDE: chess_engine::castling::CastlingRights
pub const chess_engine::castling::CastlingRights::WHITE_KINGSIDE: chess_engine::castling::CastlingRights
pub const chess_engine::castling::CastlingRights::WHITE_QUEENSIDE: chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::empty() -> Self
pub fn chess_engine::castling::CastlingRights::flip_perspective(&self) -> chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::from_fen(fen_part: &str) -> Self
pub fn chess_engine::castling::CastlingRights::has(&self, right: chess_engine::castling::CastlingRights) -> bool
pub fn chess_engine::castling::CastlingRights::new() -> Self
pub fn chess_engine::castling::CastlingRights::remove(&mut self, rights_to_remove: chess_engine::castling::CastlingRights)
pub fn chess_engine::castling::CastlingRights::to_fen(&self) -> alloc::string::String
impl core::ops::bit::BitAndAssign for chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::bitand_assign(&mut self, rhs: Self)
impl core::ops::bit::BitOr for chess_engine::castling::CastlingRights
pub type chess_engine::castling::CastlingRights::Output = chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::bitor(self, rhs: Self) -> Self::Output
impl core::ops::bit::BitOr<chess_engine::castling::CastlingRights> for u8
pub type u8::Output = chess_engine::castling::CastlingRights
pub fn u8::bitor(self, rhs: chess_engine::castling::CastlingRights) -> Self::Output
impl core::ops::bit::BitOr<u8> for chess_engine::castling::CastlingRights
pub type chess_engine::castling::CastlingRights::Output = chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::bitor(self, rhs: u8) -> Self::Output
impl core::ops::bit::BitOrAssign for chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::bitor_assign(&mut self, rhs: Self)
impl core::ops::bit::BitXor for chess_engine::castling::CastlingRights
pub type chess_engine::castling::CastlingRights::Output = chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::bitxor(self, rhs: Self) -> Self::Output
impl core::ops::bit::Not for chess_engine::castling::CastlingRights
pub type chess_engine::castling::CastlingRights::Output = chess_engine::castling::CastlingRights
pub fn chess_engine::castling::CastlingRights::not(self) -> Self::Output
pub struct chess_engine::ChessBatch<B: burn_backend::backend::base::Backend>
pub chess_engine::ChessBatch::boards: burn_tensor::tensor::api::base::Tensor<B, 3>
pub chess_engine::ChessBatch::metas: burn_tensor::tensor::api::base::Tensor<B, 2>
pub chess_engine::ChessBatch::policy_targets: burn_tensor::tensor::api::base::Tensor<B, 2>
pub chess_engine::ChessBatch::value_targets: burn_tensor::tensor::api::base::Tensor<B, 2>
impl<B: burn_backend::backend::base::Backend> burn_core::data::dataloader::batcher::Batcher<B, chess_engine::TrainingSample, chess_engine::ChessBatch<B>> for chess_engine::ChessBatcher
pub fn chess_engine::ChessBatcher::batch(&self, items: alloc::vec::Vec<chess_engine::TrainingSample>, device: &<B as burn_backend::backend::base::Backend>::Device) -> chess_engine::ChessBatch<B>
pub struct chess_engine::ChessBatcher
impl<B: burn_backend::backend::base::Backend> burn_core::data::dataloader::batcher::Batcher<B, chess_engine::TrainingSample, chess_engine::ChessBatch<B>> for chess_engine::ChessBatcher
pub fn chess_engine::ChessBatcher::batch(&self, items: alloc::vec::Vec<chess_engine::TrainingSample>, device: &<B as burn_backend::backend::base::Backend>::Device) -> chess_engine::ChessBatch<B>
pub struct chess_engine::ChessBoard
pub chess_engine::ChessBoard::all_pieces: chess_engine::bitboard::Bitboard
pub chess_engine::ChessBoard::black_occupancy: chess_engine::bitboard::Bitboard
pub chess_engine::ChessBoard::pieces: [[chess_engine::bitboard::Bitboard; 6]; 2]
pub chess_engine::ChessBoard::white_occupancy: chess_engine::bitboard::Bitboard
impl chess_engine::chess_board::ChessBoard
pub const chess_engine::chess_board::ChessBoard::BETWEEN: [[core::option::Option<chess_engine::bitboard::Bitboard>; 64]; 64]
pub const chess_engine::chess_board::ChessBoard::BISHOP_ATTACKS: [[chess_engine::bitboard::Bitboard; 4]; 64]
pub const chess_engine::chess_board::ChessBoard::BLACK_SQUARES: chess_engine::bitboard::Bitboard
pub const chess_engine::chess_board::ChessBoard::KING_ATTACKS: [chess_engine::bitboard::Bitboard; 64]
pub const chess_engine::chess_board::ChessBoard::KNIGHT_ATTACKS: [chess_engine::bitboard::Bitboard; 64]
pub const chess_engine::chess_board::ChessBoard::PAWN_ATTACKS_BLACK: [chess_engine::bitboard::Bitboard; 64]
pub const chess_engine::chess_board::ChessBoard::PAWN_ATTACKS_WHITE: [chess_engine::bitboard::Bitboard; 64]
pub const chess_engine::chess_board::ChessBoard::ROOK_ATTACKS: [[chess_engine::bitboard::Bitboard; 4]; 64]
pub const chess_engine::chess_board::ChessBoard::WHITE_SQUARES: chess_engine::bitboard::Bitboard
pub fn chess_engine::chess_board::ChessBoard::add_piece(&mut self, piece: chess_engine::chess_piece::ChessPiece, square: chess_engine::chess_square::ChessSquare)
pub fn chess_engine::chess_board::ChessBoard::apply_move(&mut self, mov: &chess_engine::chess_move::ChessMove, side_to_move: chess_engine::chess_piece::Color, en_passant_sq: core::option::Option<chess_engine::chess_square::ChessSquare>)
pub fn chess_engine::chess_board::ChessBoard::display_ascii(&self) -> alloc::string::String
pub fn chess_engine::chess_board::ChessBoard::empty() -> Self
pub fn chess_engine::chess_board::ChessBoard::flip_board(&self) -> Self
pub const fn chess_engine::chess_board::ChessBoard::generate_rook_direction_masks() -> [[chess_engine::bitboard::Bitboard; 4]; 64]
pub fn chess_engine::chess_board::ChessBoard::get_piece_at(&self, square: chess_engine::chess_square::ChessSquare) -> core::option::Option<chess_engine::chess_piece::ChessPiece>
pub fn chess_engine::chess_board::ChessBoard::get_piece_bitboard(&self, color: chess_engine::chess_piece::Color, piece_type: chess_engine::chess_piece::PieceType) -> chess_engine::bitboard::Bitboard
pub fn chess_engine::chess_board::ChessBoard::is_square_attacked(&self, sq: chess_engine::chess_square::ChessSquare, attacker_color: chess_engine::chess_piece::Color) -> bool
pub fn chess_engine::chess_board::ChessBoard::move_piece(&mut self, from_sq: chess_engine::chess_square::ChessSquare, to_sq: chess_engine::chess_square::ChessSquare, piece: chess_engine::chess_piece::ChessPiece)
pub fn chess_engine::chess_board::ChessBoard::new() -> Self
pub fn chess_engine::chess_board::ChessBoard::remove_piece(&mut self, piece: chess_engine::chess_piece::ChessPiece, square: chess_engine::chess_square::ChessSquare)
pub struct chess_engine::ChessGame
pub chess_engine::ChessGame::fullmove_counter: u32
pub chess_engine::ChessGame::game_history: alloc::vec::Vec<chess_engine::chess_position::ChessPosition>
pub chess_engine::ChessGame::outcome: chess_engine::chess_game::Outcome
pub chess_engine::ChessGame::position: chess_engine::chess_position::ChessPosition
pub chess_engine::ChessGame::rule_set: chess_engine::chess_game::RuleSet
impl chess_engine::chess_game::ChessGame
pub fn chess_engine::chess_game::ChessGame::check_game_state(&self) -> chess_engine::chess_game::Outcome
pub fn chess_engine::chess_game::ChessGame::fen_to_ascii(fen: &str)
pub fn chess_engine::chess_game::ChessGame::from_fen(fen: &str) -> Self
pub fn chess_engine::chess_game::ChessGame::make_move(&mut self, mov: &chess_engine::chess_move::ChessMove)
pub fn chess_engine::chess_game::ChessGame::to_fen(&self) -> alloc::string::String
pub fn chess_engine::chess_game::ChessGame::uci_to_move(&self, input: &str) -> core::result::Result<chess_engine::chess_move::ChessMove, &str>
pub fn chess_engine::chess_game::ChessGame::unmake_move(&mut self)
impl core::default::Default for chess_engine::chess_game::ChessGame
pub fn chess_engine::chess_game::ChessGame::default() -> Self
pub struct chess_engine::ChessMove
pub chess_engine::ChessMove::from: chess_engine::chess_square::ChessSquare
pub chess_engine::ChessMove::promotion: core::option::Option<chess_engine::chess_piece::PieceType>
pub chess_engine::ChessMove::to: chess_engine::chess_square::ChessSquare
impl chess_engine::chess_move::ChessMove
pub fn chess_engine::chess_move::ChessMove::from_uci(uci: &str) -> core::result::Result<Self, &'static str>
pub fn chess_engine::chess_move::ChessMove::new(from: chess_engine::chess_square::ChessSquare, to: chess_engine::chess_square::ChessSquare, promotion: core::option::Option<chess_engine::chess_piece::PieceType>) -> Self
pub fn chess_engine::chess_move::ChessMove::to_uci(&self) -> alloc::string::String
pub struct chess_engine::ChessPiece
pub chess_engine::ChessPiece::color: chess_engine::chess_piece::Color
pub chess_engine::ChessPiece::piece_type: chess_engine::chess_piece::PieceType
impl chess_engine::chess_piece::ChessPiece
pub fn chess_engine::chess_piece::ChessPiece::new(color: chess_engine::chess_piece::Color, piece_type: chess_engine::chess_piece::PieceType) -> Self
pub struct chess_engine::ChessPosition
pub chess_engine::ChessPosition::castling_rights: chess_engine::castling::CastlingRights
pub chess_engine::ChessPosition::chessboard: chess_engine::chess_board::ChessBoard
pub chess_engine::ChessPosition::en_passant: core::option::Option<chess_engine::chess_square::ChessSquare>
pub chess_engine::ChessPosition::halfmove_clock: u32
pub chess_engine::ChessPosition::pseudolegal_moves: alloc::vec::Vec<chess_engine::chess_move::ChessMove>
pub chess_engine::ChessPosition::side_to_move: chess_engine::chess_piece::Color
pub chess_engine::ChessPosition::zobrist_hash: u64
impl chess_engine::chess_position::ChessPosition
pub fn chess_engine::chess_position::ChessPosition::calculate_hash(&self) -> u64
pub fn chess_engine::chess_position::ChessPosition::check_game_state(&self, rule_set: chess_engine::chess_game::RuleSet) -> chess_engine::chess_game::Outcome
pub fn chess_engine::chess_position::ChessPosition::generate_pseudolegal(&self) -> alloc::vec::Vec<chess_engine::chess_move::ChessMove>
pub fn chess_engine::chess_position::ChessPosition::get_squares(&self) -> (alloc::vec::Vec<chess_engine::chess_square::ChessSquare>, alloc::vec::Vec<chess_engine::chess_square::ChessSquare>)
pub fn chess_engine::chess_position::ChessPosition::is_geometrically_valid(&self, mov: &chess_engine::chess_move::ChessMove) -> bool
pub fn chess_engine::chess_position::ChessPosition::is_legal(&self, mov: &chess_engine::chess_move::ChessMove) -> bool
pub fn chess_engine::chess_position::ChessPosition::make_move(&mut self, mov: &chess_engine::chess_move::ChessMove)
pub struct chess_engine::ChessSquare(pub u8)
impl chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::A8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::B8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::C8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::D8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::E8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::F8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::G8: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H1: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H2: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H3: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H4: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H5: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H6: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H7: chess_engine::chess_square::ChessSquare
pub const chess_engine::chess_square::ChessSquare::H8: chess_engine::chess_square::ChessSquare
pub fn chess_engine::chess_square::ChessSquare::bitboard(self) -> chess_engine::bitboard::Bitboard
pub fn chess_engine::chess_square::ChessSquare::colour(self) -> chess_engine::chess_piece::Color
pub fn chess_engine::chess_square::ChessSquare::file(&self) -> u8
pub const fn chess_engine::chess_square::ChessSquare::from_coords(file: u8, rank: u8) -> core::option::Option<Self>
pub fn chess_engine::chess_square::ChessSquare::from_name(name: &str) -> core::option::Option<Self>
pub fn chess_engine::chess_square::ChessSquare::index(&self) -> u8
pub fn chess_engine::chess_square::ChessSquare::is_valid(self) -> bool
pub fn chess_engine::chess_square::ChessSquare::name(&self) -> alloc::string::String
pub const fn chess_engine::chess_square::ChessSquare::new(index: u8) -> core::option::Option<Self>
pub fn chess_engine::chess_square::ChessSquare::rank(&self) -> u8
pub fn chess_engine::chess_square::ChessSquare::square_north(self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub fn chess_engine::chess_square::ChessSquare::square_opposite(&self) -> chess_engine::chess_square::ChessSquare
pub fn chess_engine::chess_square::ChessSquare::square_south(self) -> core::option::Option<chess_engine::chess_square::ChessSquare>
pub fn chess_engine::chess_square::ChessSquare::to_name(&self) -> alloc::string::String
impl core::fmt::Display for chess_engine::chess_square::ChessSquare
pub fn chess_engine::chess_square::ChessSquare::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
pub struct chess_engine::ChessTransformer<B: burn_backend::backend::base::Backend>
impl<B: burn_backend::backend::base::Backend> chess_engine::model::ChessTransformer<B>
pub fn chess_engine::model::ChessTransformer<B>::forward(&self, board: burn_tensor::tensor::api::base::Tensor<B, 3>, meta: burn_tensor::tensor::api::base::Tensor<B, 2>) -> (burn_tensor::tensor::api::base::Tensor<B, 2>, burn_tensor::tensor::api::base::Tensor<B, 2>)
pub fn chess_engine::model::ChessTransformer<B>::forward_classification(&self, batch: chess_engine::ChessBatch<B>) -> burn_train::learner::classification::ClassificationOutput<B>
impl<B: burn_backend::backend::base::AutodiffBackend> burn_train::learner::train_val::TrainStep for chess_engine::model::ChessTransformer<B>
pub type chess_engine::model::ChessTransformer<B>::Input = chess_engine::ChessBatch<B>
pub type chess_engine::model::ChessTransformer<B>::Output = burn_train::learner::classification::ClassificationOutput<B>
pub fn chess_engine::model::ChessTransformer<B>::step(&self, batch: chess_engine::ChessBatch<B>) -> burn_train::learner::train_val::TrainOutput<burn_train::learner::classification::ClassificationOutput<B>>
impl<B: burn_backend::backend::base::Backend> burn_core::module::base::Module<B> for chess_engine::model::ChessTransformer<B>
pub type chess_engine::model::ChessTransformer<B>::Record = chess_engine::model::ChessTransformerRecord<B>
pub fn chess_engine::model::ChessTransformer<B>::collect_devices(&self, devices: burn_core::module::base::Devices<B>) -> burn_core::module::base::Devices<B>
pub fn chess_engine::model::ChessTransformer<B>::fork(self, device: &<B as burn_backend::backend::base::Backend>::Device) -> Self
pub fn chess_engine::model::ChessTransformer<B>::into_record(self) -> Self::Record
pub fn chess_engine::model::ChessTransformer<B>::load_record(self, record: Self::Record) -> Self
pub fn chess_engine::model::ChessTransformer<B>::map<Mapper: burn_core::module::base::ModuleMapper<B>>(self, mapper: &mut Mapper) -> Self
pub fn chess_engine::model::ChessTransformer<B>::num_params(&self) -> usize
pub fn chess_engine::model::ChessTransformer<B>::to_device(self, device: &<B as burn_backend::backend::base::Backend>::Device) -> Self
pub fn chess_engine::model::ChessTransformer<B>::visit<Visitor: burn_core::module::base::ModuleVisitor<B>>(&self, visitor: &mut Visitor)
impl<B: burn_backend::backend::base::Backend> burn_core::module::display::ModuleDisplay for chess_engine::model::ChessTransformer<B>
impl<B: burn_backend::backend::base::Backend> burn_core::module::display::ModuleDisplayDefault for chess_engine::model::ChessTransformer<B>
pub fn chess_engine::model::ChessTransformer<B>::content(&self, content: burn_core::module::display::Content) -> core::option::Option<burn_core::module::display::Content>
pub fn chess_engine::model::ChessTransformer<B>::num_params(&self) -> usize
impl<B: burn_backend::backend::base::Backend> burn_train::learner::train_val::InferenceStep for chess_engine::model::ChessTransformer<B>
pub type chess_engine::model::ChessTransformer<B>::Input = chess_engine::ChessBatch<B>
pub type chess_engine::model::ChessTransformer<B>::Output = burn_train::learner::classification::ClassificationOutput<B>
pub fn chess_engine::model::ChessTransformer<B>::step(&self, batch: chess_engine::ChessBatch<B>) -> burn_train::learner::classification::ClassificationOutput<B>
impl<B: burn_backend::backend::base::Backend> core::clone::Clone for chess_engine::model::ChessTransformer<B>
pub fn chess_engine::model::ChessTransformer<B>::clone(&self) -> Self
impl<B: burn_backend::backend::base::Backend> core::fmt::Display for chess_engine::model::ChessTransformer<B>
pub fn chess_engine::model::ChessTransformer<B>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<B> burn_core::module::base::AutodiffModule<B> for chess_engine::model::ChessTransformer<B> where B: burn_backend::backend::base::AutodiffBackend + burn_backend::backend::base::Backend, <B as burn_backend::backend::base::AutodiffBackend>::InnerBackend: burn_backend::backend::base::Backend
pub type chess_engine::model::ChessTransformer<B>::InnerModule = chess_engine::model::ChessTransformer<<B as burn_backend::backend::base::AutodiffBackend>::InnerBackend>
pub fn chess_engine::model::ChessTransformer<B>::from_inner(module: Self::InnerModule) -> Self
pub fn chess_engine::model::ChessTransformer<B>::valid(&self) -> Self::InnerModule
impl<B> burn_core::module::base::HasAutodiffModule<B> for chess_engine::model::ChessTransformer<<B as burn_backend::backend::base::AutodiffBackend>::InnerBackend> where B: burn_backend::backend::base::AutodiffBackend + burn_backend::backend::base::Backend, <B as burn_backend::backend::base::AutodiffBackend>::InnerBackend: burn_backend::backend::base::Backend
pub type chess_engine::model::ChessTransformer<<B as burn_backend::backend::base::AutodiffBackend>::InnerBackend>::TrainModule = chess_engine::model::ChessTransformer<B>
pub struct chess_engine::MctsConfig
pub struct chess_engine::MctsEdge
pub chess_engine::MctsEdge::child: core::option::Option<usize>
pub chess_engine::MctsEdge::confidence: f32
pub chess_engine::MctsEdge::mean_value: [f32; 3]
pub chess_engine::MctsEdge::square: chess_engine::chess_square::ChessSquare
pub chess_engine::MctsEdge::total_value: [f32; 3]
pub chess_engine::MctsEdge::visits: u32
impl chess_engine::MctsEdge
pub fn chess_engine::MctsEdge::get_puct(&self, c_puct: f32, total_parent_visits: u32, side_to_move: chess_engine::chess_piece::Color) -> f32
pub fn chess_engine::MctsEdge::new(sq: chess_engine::chess_square::ChessSquare, confidence: f32) -> Self
pub fn chess_engine::MctsEdge::update(&mut self, value: [f32; 3])
pub struct chess_engine::MctsNodeAction<'a>
impl<'a> chess_engine::MctsNodeAction<'a>
pub fn chess_engine::MctsNodeAction<'a>::new(mcts_state: &'a chess_engine::MctsNodeState, selected_sq: chess_engine::chess_square::ChessSquare, parent: usize) -> Self
pub struct chess_engine::MctsNodeState
impl chess_engine::MctsNodeState
pub fn chess_engine::MctsNodeState::new(game: &chess_engine::chess_game::ChessGame, parent: core::option::Option<usize>) -> Self
pub struct chess_engine::NetworkInputs
pub chess_engine::NetworkInputs::boards: [f32; 896]
pub chess_engine::NetworkInputs::meta: [f32; 5]
impl chess_engine::NetworkInputs
pub fn chess_engine::NetworkInputs::from_position(position: &chess_engine::chess_position::ChessPosition, selected_sq: core::option::Option<chess_engine::chess_square::ChessSquare>) -> Self
pub fn chess_engine::NetworkInputs::new(position: &chess_engine::chess_position::ChessPosition) -> Self
impl core::default::Default for chess_engine::NetworkInputs
pub fn chess_engine::NetworkInputs::default() -> Self
pub struct chess_engine::NetworkLabels
pub chess_engine::NetworkLabels::policy: [f32; 64]
pub chess_engine::NetworkLabels::value: [f32; 3]
impl chess_engine::NetworkLabels
pub fn chess_engine::NetworkLabels::as_squares(&self) -> [(chess_engine::chess_square::ChessSquare, f32); 64]
impl core::default::Default for chess_engine::NetworkLabels
pub fn chess_engine::NetworkLabels::default() -> Self
pub struct chess_engine::ReplayBuffer
pub chess_engine::ReplayBuffer::buffer: alloc::vec::Vec<chess_engine::TrainingSample>
pub chess_engine::ReplayBuffer::capacity: usize
pub chess_engine::ReplayBuffer::pointer: usize
impl chess_engine::ReplayBuffer
pub fn chess_engine::ReplayBuffer::new(capacity: usize) -> Self
pub fn chess_engine::ReplayBuffer::push(&mut self, sample: &chess_engine::TrainingSample)
pub fn chess_engine::ReplayBuffer::sample_batch<B: burn_backend::backend::base::Backend>(&self, batch_size: usize, rng: &mut rand::rngs::small::SmallRng, device: &<B as burn_backend::backend::base::Backend>::Device) -> chess_engine::ChessBatch<B>
pub struct chess_engine::TrainingConfig
pub chess_engine::TrainingConfig::batch_size: usize
pub chess_engine::TrainingConfig::learning_rate: f64
pub chess_engine::TrainingConfig::legal: bool
pub chess_engine::TrainingConfig::masked: bool
pub chess_engine::TrainingConfig::model: chess_engine::model::ChessTransformerConfig
pub chess_engine::TrainingConfig::num_epochs: usize
pub chess_engine::TrainingConfig::num_workers: usize
pub chess_engine::TrainingConfig::optimizer: burn_optim::optim::adam::AdamConfig
pub chess_engine::TrainingConfig::seed: u64
impl chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::new(model: chess_engine::model::ChessTransformerConfig, masked: bool, legal: bool, optimizer: burn_optim::optim::adam::AdamConfig) -> Self
impl chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::with_batch_size(self, batch_size: usize) -> Self
pub fn chess_engine::TrainingConfig::with_learning_rate(self, learning_rate: f64) -> Self
pub fn chess_engine::TrainingConfig::with_num_epochs(self, num_epochs: usize) -> Self
pub fn chess_engine::TrainingConfig::with_num_workers(self, num_workers: usize) -> Self
pub fn chess_engine::TrainingConfig::with_seed(self, seed: u64) -> Self
impl burn_core::config::Config for chess_engine::TrainingConfig
impl core::clone::Clone for chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::clone(&self) -> Self
impl core::fmt::Display for chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl serde_core::ser::Serialize for chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::serialize<S>(&self, serializer: S) -> core::result::Result<<S as serde_core::ser::Serializer>::Ok, <S as serde_core::ser::Serializer>::Error> where S: serde_core::ser::Serializer
impl<'de> serde_core::de::Deserialize<'de> for chess_engine::TrainingConfig
pub fn chess_engine::TrainingConfig::deserialize<D>(deserializer: D) -> core::result::Result<Self, <D as serde_core::de::Deserializer>::Error> where D: serde_core::de::Deserializer<'de>
pub struct chess_engine::TrainingSample
pub chess_engine::TrainingSample::inputs: chess_engine::NetworkInputs
pub chess_engine::TrainingSample::targets: chess_engine::NetworkLabels
impl core::default::Default for chess_engine::TrainingSample
pub fn chess_engine::TrainingSample::default() -> Self
impl<B: burn_backend::backend::base::Backend> burn_core::data::dataloader::batcher::Batcher<B, chess_engine::TrainingSample, chess_engine::ChessBatch<B>> for chess_engine::ChessBatcher
pub fn chess_engine::ChessBatcher::batch(&self, items: alloc::vec::Vec<chess_engine::TrainingSample>, device: &<B as burn_backend::backend::base::Backend>::Device) -> chess_engine::ChessBatch<B>
pub struct chess_engine::ZobristKeys
pub chess_engine::ZobristKeys::castling: [u64; 16]
pub chess_engine::ZobristKeys::en_passant: [u64; 8]
pub chess_engine::ZobristKeys::pieces: [[[u64; 64]; 6]; 2]
pub chess_engine::ZobristKeys::side_to_move: u64
impl chess_engine::zobrist::ZobristKeys
pub fn chess_engine::zobrist::ZobristKeys::get() -> &'static Self
pub fn chess_engine::zobrist::ZobristKeys::new() -> Self
pub fn chess_engine::expand<B: burn_backend::backend::base::Backend>(nodes: &mut alloc::vec::Vec<&impl Node>, model: chess_engine::model::ChessTransformer<B>, config: &chess_engine::TrainingConfig, device: &<B as burn_backend::backend::base::Backend>::Device)
pub fn chess_engine::generate_self_play_data<B: burn_backend::backend::base::AutodiffBackend>(artifact_dir: &str, training_config: &chess_engine::TrainingConfig, device: &<B as burn_backend::backend::base::Backend>::Device) -> alloc::vec::Vec<chess_engine::TrainingSample>
pub fn chess_engine::inputs_to_tensor<B: burn_backend::backend::base::Backend>(buffer: &alloc::vec::Vec<chess_engine::NetworkInputs>, device: &<B as burn_backend::backend::base::Backend>::Device) -> (burn_tensor::tensor::api::base::Tensor<B, 3>, burn_tensor::tensor::api::base::Tensor<B, 2>)
pub fn chess_engine::model_make_outputs<B: burn_backend::backend::base::Backend>(model: chess_engine::model::ChessTransformer<B>, inputs: &alloc::vec::Vec<chess_engine::NetworkInputs>, config: &chess_engine::TrainingConfig, masks: core::option::Option<alloc::vec::Vec<bool>>, device: &<B as burn_backend::backend::base::Backend>::Device) -> alloc::vec::Vec<chess_engine::NetworkLabels>
pub fn chess_engine::run_mcts<B: burn_backend::backend::base::Backend>(games: &alloc::vec::Vec<chess_engine::chess_game::ChessGame>, model: chess_engine::model::ChessTransformer<B>, mcts_config: &chess_engine::MctsConfig, training_config: &chess_engine::TrainingConfig, device: &<B as burn_backend::backend::base::Backend>::Device)
pub fn chess_engine::train<B: burn_backend::backend::base::AutodiffBackend>(model: chess_engine::model::ChessTransformer<B>, optimizer: &mut burn_optim::optim::simple::adaptor::OptimizerAdaptor<burn_optim::optim::adam::Adam, chess_engine::model::ChessTransformer<B>, B>, config: chess_engine::TrainingConfig, games: &chess_engine::ReplayBuffer, device: &<B as burn_backend::backend::base::Backend>::Device, rng: &mut rand::rngs::small::SmallRng)

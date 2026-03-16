pub use burn
pub struct Bitboard(pub u64)
impl Bitboard
pub const ALL: Bitboard
pub const ALL_PIECES: Bitboard
pub const BLACK_BISHOPS: Bitboard
pub const BLACK_KING: Bitboard
pub const BLACK_KNIGHTS: Bitboard
pub const BLACK_OCCUPANCY: Bitboard
pub const BLACK_PAWNS: Bitboard
pub const BLACK_QUEENS: Bitboard
pub const BLACK_ROOKS: Bitboard
pub const EMPTY: Bitboard
pub const WHITE_BISHOPS: Bitboard
pub const WHITE_KING: Bitboard
pub const WHITE_KNIGHTS: Bitboard
pub const WHITE_OCCUPANCY: Bitboard
pub const WHITE_PAWNS: Bitboard
pub const WHITE_QUEENS: Bitboard
pub const WHITE_ROOKS: Bitboard
pub fn clear(&mut self, square: ChessSquare)
pub fn count(&self) -> u32
pub fn flip(&mut self)
pub fn flipped(&self) -> Bitboard
pub fn from_square(square: ChessSquare) -> Self
pub fn is_empty(&self) -> bool
pub fn is_set(&self, square: ChessSquare) -> bool
pub fn lsb_square(&self) -> Option<ChessSquare>
pub fn msb_square(&self) -> Option<ChessSquare>
pub fn pop_lsb(&mut self) -> Option<ChessSquare>
pub fn pop_msb(&mut self) -> Option<ChessSquare>
pub const fn set(&mut self, square: ChessSquare)
pub fn shift_east(self) -> Self
pub fn shift_north(self) -> Self
pub fn shift_north_east(self) -> Self
pub fn shift_north_west(self) -> Self
pub fn shift_south(self) -> Self
pub fn shift_south_east(self) -> Self
pub fn shift_south_west(self) -> Self
pub fn shift_west(self) -> Self
pub fn to_bool(&self) -> [bool; 64]
pub fn toggle(&mut self, square: ChessSquare)
pub fn write_to_slice(&self, slice: &mut [f32])
impl fmt::Display for Bitboard
pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
impl ops::bit::BitAnd for Bitboard
pub type Output = Bitboard
pub fn bitand(self, rhs: Self) -> Self::Output
impl ops::bit::BitAnd<&Bitboard> for &Bitboard
pub type &Output = Bitboard
pub fn &bitand(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitAnd<&Bitboard> for Bitboard
pub fn bitand(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitAnd<Bitboard> for &Bitboard
pub fn &bitand(self, rhs: Bitboard) -> Self::Output
impl ops::bit::BitAndAssign for Bitboard
pub fn bitand_assign(&mut self, rhs: Self)
impl ops::bit::BitOr for Bitboard
pub fn bitor(self, rhs: Self) -> Self::Output
impl ops::bit::BitOr<&Bitboard> for &Bitboard
pub fn &bitor(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitOr<&Bitboard> for Bitboard
pub fn bitor(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitOr<Bitboard> for &Bitboard
pub fn &bitor(self, rhs: Bitboard) -> Self::Output
impl ops::bit::BitOrAssign for Bitboard
pub fn bitor_assign(&mut self, rhs: Self)
impl ops::bit::BitXor for Bitboard
pub fn bitxor(self, rhs: Self) -> Self::Output
impl ops::bit::BitXor<&Bitboard> for &Bitboard
pub fn &bitxor(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitXor<&Bitboard> for Bitboard
pub fn bitxor(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitXor<Bitboard> for &Bitboard
pub fn &bitxor(self, rhs: Bitboard) -> Self::Output
impl ops::bit::BitXorAssign for Bitboard
pub fn bitxor_assign(&mut self, rhs: Self)
impl ops::bit::Not for Bitboard
pub fn not(self) -> Self::Output
pub struct castling::CastlingRights(pub u8)
impl castling::CastlingRights
pub const castling::CastlingRights::BLACK_KINGSIDE: castling::CastlingRights
pub const castling::CastlingRights::BLACK_QUEENSIDE: castling::CastlingRights
pub const castling::CastlingRights::WHITE_KINGSIDE: castling::CastlingRights
pub const castling::CastlingRights::WHITE_QUEENSIDE: castling::CastlingRights
pub fn castling::CastlingRights::empty() -> Self
pub fn castling::CastlingRights::flip_perspective(&self) -> castling::CastlingRights
pub fn castling::CastlingRights::from_fen(fen_part: &str) -> Self
pub fn castling::CastlingRights::has(&self, right: castling::CastlingRights) -> bool
pub fn castling::CastlingRights::new() -> Self
pub fn castling::CastlingRights::remove(&mut self, rights_to_remove: castling::CastlingRights)
pub fn castling::CastlingRights::to_fen(&self) -> alloc::string::String
impl ops::bit::BitAndAssign for castling::CastlingRights
pub fn castling::CastlingRights::bitand_assign(&mut self, rhs: Self)
impl ops::bit::BitOr for castling::CastlingRights
pub type castling::CastlingRights::Output = castling::CastlingRights
pub fn castling::CastlingRights::bitor(self, rhs: Self) -> Self::Output
impl ops::bit::BitOr<castling::CastlingRights> for u8
pub type u8::Output = castling::CastlingRights
pub fn u8::bitor(self, rhs: castling::CastlingRights) -> Self::Output
impl ops::bit::BitOr<u8> for castling::CastlingRights
pub fn castling::CastlingRights::bitor(self, rhs: u8) -> Self::Output
impl ops::bit::BitOrAssign for castling::CastlingRights
pub fn castling::CastlingRights::bitor_assign(&mut self, rhs: Self)
impl ops::bit::BitXor for castling::CastlingRights
pub fn castling::CastlingRights::bitxor(self, rhs: Self) -> Self::Output
impl ops::bit::Not for castling::CastlingRights
pub fn castling::CastlingRights::not(self) -> Self::Output
pub struct chess_board::ChessBoard
pub chess_board::ChessBoard::all_pieces: Bitboard
pub chess_board::ChessBoard::black_occupancy: Bitboard
pub chess_board::ChessBoard::pieces: [[Bitboard; 6]; 2]
pub chess_board::ChessBoard::white_occupancy: Bitboard
impl chess_board::ChessBoard
pub const chess_board::ChessBoard::BETWEEN: [[Option<Bitboard>; 64]; 64]
pub const chess_board::ChessBoard::BISHOP_ATTACKS: [[Bitboard; 4]; 64]
pub const chess_board::ChessBoard::BLACK_SQUARES: Bitboard
pub const chess_board::ChessBoard::KING_ATTACKS: [Bitboard; 64]
pub const chess_board::ChessBoard::KNIGHT_ATTACKS: [Bitboard; 64]
pub const chess_board::ChessBoard::PAWN_ATTACKS_BLACK: [Bitboard; 64]
pub const chess_board::ChessBoard::PAWN_ATTACKS_WHITE: [Bitboard; 64]
pub const chess_board::ChessBoard::ROOK_ATTACKS: [[Bitboard; 4]; 64]
pub const chess_board::ChessBoard::WHITE_SQUARES: Bitboard
pub fn chess_board::ChessBoard::add_piece(&mut self, piece: chess_piece::ChessPiece, square: ChessSquare)
pub fn chess_board::ChessBoard::apply_move(&mut self, mov: &chess_move::ChessMove, side_to_move: chess_piece::Color, en_passant_sq: Option<ChessSquare>)
pub fn chess_board::ChessBoard::display_ascii(&self) -> alloc::string::String
pub fn chess_board::ChessBoard::empty() -> Self
pub fn chess_board::ChessBoard::flip_board(&self) -> Self
pub const fn chess_board::ChessBoard::generate_rook_direction_masks() -> [[Bitboard; 4]; 64]
pub fn chess_board::ChessBoard::get_piece_at(&self, square: ChessSquare) -> Option<chess_piece::ChessPiece>
pub fn chess_board::ChessBoard::get_piece_bitboard(&self, color: chess_piece::Color, piece_type: chess_piece::PieceType) -> Bitboard
pub fn chess_board::ChessBoard::is_square_attacked(&self, sq: ChessSquare, attacker_color: chess_piece::Color) -> bool
pub fn chess_board::ChessBoard::move_piece(&mut self, from_sq: ChessSquare, to_sq: ChessSquare, piece: chess_piece::ChessPiece)
pub fn chess_board::ChessBoard::new() -> Self
pub fn chess_board::ChessBoard::remove_piece(&mut self, piece: chess_piece::ChessPiece, square: ChessSquare)
pub enum chess_game::Outcome
pub chess_game::Outcome::Finished(Option<chess_piece::Color>)
pub chess_game::Outcome::Unfinished
pub enum chess_game::RuleSet
pub chess_game::RuleSet::Legal
pub chess_game::RuleSet::PseudoLegal
pub struct chess_game::ChessGame
pub chess_game::ChessGame::castling_rights: castling::CastlingRights
pub chess_game::ChessGame::chessboard: chess_board::ChessBoard
pub chess_game::ChessGame::en_passant: Option<ChessSquare>
pub chess_game::ChessGame::fullmove_counter: u32
pub chess_game::ChessGame::game_history: alloc::vec::Vec<chess_game::GameStateEntry>
pub chess_game::ChessGame::halfmove_clock: u32
pub chess_game::ChessGame::outcome: chess_game::Outcome
pub chess_game::ChessGame::rule_set: chess_game::RuleSet
pub chess_game::ChessGame::side_to_move: chess_piece::Color
pub chess_game::ChessGame::zobrist_hash: u64
impl chess_game::ChessGame
pub fn chess_game::ChessGame::calculate_hash(&mut self) -> u64
pub fn chess_game::ChessGame::check_game_state(&self) -> chess_game::Outcome
pub fn chess_game::ChessGame::fen_to_ascii(fen: &str)
pub fn chess_game::ChessGame::from_fen(fen: &str) -> Self
pub fn chess_game::ChessGame::generate_pseudolegal(&self) -> alloc::vec::Vec<chess_move::ChessMove>
pub fn chess_game::ChessGame::is_legal(&self, mov: &chess_move::ChessMove) -> bool
pub fn chess_game::ChessGame::make_move(&mut self, mov: &chess_move::ChessMove)
pub fn chess_game::ChessGame::to_fen(&self) -> alloc::string::String
pub fn chess_game::ChessGame::uci_to_move(&self, input: &str) -> result::Result<chess_move::ChessMove, &str>
impl default::Default for chess_game::ChessGame
pub fn chess_game::ChessGame::default() -> Self
pub struct chess_game::GameStateEntry
pub chess_game::GameStateEntry::castling_rights: castling::CastlingRights
pub chess_game::GameStateEntry::chessboard: chess_board::ChessBoard
pub chess_game::GameStateEntry::en_passant: Option<ChessSquare>
pub chess_game::GameStateEntry::fullmove_counter: u32
pub chess_game::GameStateEntry::halfmove_clock: u32
pub chess_game::GameStateEntry::side_to_move: chess_piece::Color
pub chess_game::GameStateEntry::zobrist_hash: u64
pub struct chess_move::ChessMove
pub chess_move::ChessMove::from: ChessSquare
pub chess_move::ChessMove::promotion: Option<chess_piece::PieceType>
pub chess_move::ChessMove::to: ChessSquare
impl chess_move::ChessMove
pub fn chess_move::ChessMove::from_uci(uci: &str) -> result::Result<Self, &'static str>
pub fn chess_move::ChessMove::new(from: ChessSquare, to: ChessSquare, promotion: Option<chess_piece::PieceType>) -> Self
pub fn chess_move::ChessMove::to_uci(&self) -> alloc::string::String
#[repr(usize)] pub enum chess_piece::Color
pub chess_piece::Color::Black = 1
pub chess_piece::Color::White = 0
impl chess_piece::Color
pub fn chess_piece::Color::from_char(c: char) -> Option<Self>
pub fn chess_piece::Color::opposite(&self) -> Self
pub fn chess_piece::Color::to_char(&self) -> char
#[repr(usize)] pub enum chess_piece::PieceType
pub chess_piece::PieceType::Bishop = 2
pub chess_piece::PieceType::King = 5
pub chess_piece::PieceType::Knight = 1
pub chess_piece::PieceType::Pawn = 0
pub chess_piece::PieceType::Queen = 4
pub chess_piece::PieceType::Rook = 3
impl chess_piece::PieceType
pub fn chess_piece::PieceType::from_char(c: char) -> Option<Self>
pub fn chess_piece::PieceType::from_idx(idx: usize) -> Option<chess_piece::PieceType>
pub fn chess_piece::PieceType::to_char(&self, color: chess_piece::Color) -> char
pub struct chess_piece::ChessPiece
pub chess_piece::ChessPiece::color: chess_piece::Color
pub chess_piece::ChessPiece::piece_type: chess_piece::PieceType
impl chess_piece::ChessPiece
pub fn chess_piece::ChessPiece::new(color: chess_piece::Color, piece_type: chess_piece::PieceType) -> Self
pub struct ChessSquare(pub u8)
impl ChessSquare
pub const ChessSquare::A1: ChessSquare
pub const ChessSquare::A2: ChessSquare
pub const ChessSquare::A3: ChessSquare
pub const ChessSquare::A4: ChessSquare
pub const ChessSquare::A5: ChessSquare
pub const ChessSquare::A6: ChessSquare
pub const ChessSquare::A7: ChessSquare
pub const ChessSquare::A8: ChessSquare
pub const ChessSquare::B1: ChessSquare
pub const ChessSquare::B2: ChessSquare
pub const ChessSquare::B3: ChessSquare
pub const ChessSquare::B4: ChessSquare
pub const ChessSquare::B5: ChessSquare
pub const ChessSquare::B6: ChessSquare
pub const ChessSquare::B7: ChessSquare
pub const ChessSquare::B8: ChessSquare
pub const ChessSquare::C1: ChessSquare
pub const ChessSquare::C2: ChessSquare
pub const ChessSquare::C3: ChessSquare
pub const ChessSquare::C4: ChessSquare
pub const ChessSquare::C5: ChessSquare
pub const ChessSquare::C6: ChessSquare
pub const ChessSquare::C7: ChessSquare
pub const ChessSquare::C8: ChessSquare
pub const ChessSquare::D1: ChessSquare
pub const ChessSquare::D2: ChessSquare
pub const ChessSquare::D3: ChessSquare
pub const ChessSquare::D4: ChessSquare
pub const ChessSquare::D5: ChessSquare
pub const ChessSquare::D6: ChessSquare
pub const ChessSquare::D7: ChessSquare
pub const ChessSquare::D8: ChessSquare
pub const ChessSquare::E1: ChessSquare
pub const ChessSquare::E2: ChessSquare
pub const ChessSquare::E3: ChessSquare
pub const ChessSquare::E4: ChessSquare
pub const ChessSquare::E5: ChessSquare
pub const ChessSquare::E6: ChessSquare
pub const ChessSquare::E7: ChessSquare
pub const ChessSquare::E8: ChessSquare
pub const ChessSquare::F1: ChessSquare
pub const ChessSquare::F2: ChessSquare
pub const ChessSquare::F3: ChessSquare
pub const ChessSquare::F4: ChessSquare
pub const ChessSquare::F5: ChessSquare
pub const ChessSquare::F6: ChessSquare
pub const ChessSquare::F7: ChessSquare
pub const ChessSquare::F8: ChessSquare
pub const ChessSquare::G1: ChessSquare
pub const ChessSquare::G2: ChessSquare
pub const ChessSquare::G3: ChessSquare
pub const ChessSquare::G4: ChessSquare
pub const ChessSquare::G5: ChessSquare
pub const ChessSquare::G6: ChessSquare
pub const ChessSquare::G7: ChessSquare
pub const ChessSquare::G8: ChessSquare
pub const ChessSquare::H1: ChessSquare
pub const ChessSquare::H2: ChessSquare
pub const ChessSquare::H3: ChessSquare
pub const ChessSquare::H4: ChessSquare
pub const ChessSquare::H5: ChessSquare
pub const ChessSquare::H6: ChessSquare
pub const ChessSquare::H7: ChessSquare
pub const ChessSquare::H8: ChessSquare
pub fn ChessSquare::bitboard(self) -> Bitboard
pub fn ChessSquare::colour(self) -> chess_piece::Color
pub fn ChessSquare::file(&self) -> u8
pub const fn ChessSquare::from_coords(file: u8, rank: u8) -> Option<Self>
pub fn ChessSquare::from_name(name: &str) -> Option<Self>
pub fn ChessSquare::index(&self) -> u8
pub fn ChessSquare::is_valid(self) -> bool
pub fn ChessSquare::name(&self) -> alloc::string::String
pub const fn ChessSquare::new(index: u8) -> Option<Self>
pub fn ChessSquare::rank(&self) -> u8
pub fn ChessSquare::square_north(self) -> Option<ChessSquare>
pub fn ChessSquare::square_opposite(&self) -> ChessSquare
pub fn ChessSquare::square_south(self) -> Option<ChessSquare>
pub fn ChessSquare::to_name(&self) -> alloc::string::String
impl fmt::Display for ChessSquare
pub fn ChessSquare::fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
pub struct ChessBatch<B: burn_backend::backend::base::Backend>
pub ChessBatch::boards: burn_tensor::tensor::api::base::Tensor<B, 3>
pub ChessBatch::metas: burn_tensor::tensor::api::base::Tensor<B, 2>
pub ChessBatch::policy_targets: burn_tensor::tensor::api::base::Tensor<B, 2>
pub ChessBatch::value_targets: burn_tensor::tensor::api::base::Tensor<B, 2>
impl<B: burn_backend::backend::base::Backend> burn_dataloader::batcher::Batcher<B, TrainingSample, ChessBatch<B>> for ChessBatcher
pub fn ChessBatcher::batch(&self, items: alloc::vec::Vec<TrainingSample>, device: &<B as burn_backend::backend::base::Backend>::Device) -> ChessBatch<B>
pub struct ChessBatcher
pub struct ReplayBuffer
pub ReplayBuffer::buffer: alloc::vec::Vec<TrainingSample>
pub ReplayBuffer::capacity: usize
pub ReplayBuffer::pointer: usize
impl ReplayBuffer
pub fn ReplayBuffer::new(capacity: usize) -> Self
pub fn ReplayBuffer::push(&mut self, sample: &TrainingSample)
pub fn ReplayBuffer::sample_batch<B: burn_backend::backend::base::Backend>(&self, batch_size: usize, rng: &mut rand::rngs::small::SmallRng, device: &<B as burn_backend::backend::base::Backend>::Device) -> ChessBatch<B>
pub struct TrainingSample
pub TrainingSample::board: [f32; 896]
pub TrainingSample::meta: [f32; 5]
pub TrainingSample::target_policy: [f32; 64]
pub TrainingSample::target_value: [f32; 3]
impl default::Default for TrainingSample
pub fn TrainingSample::default() -> Self
pub struct engine::TrainingConfig
pub engine::TrainingConfig::batch_size: usize
pub engine::TrainingConfig::learning_rate: f64
pub engine::TrainingConfig::masked: bool
pub engine::TrainingConfig::num_epochs: usize
pub engine::TrainingConfig::num_workers: usize
pub engine::TrainingConfig::optimizer: burn_optim::optim::adam::AdamConfig
pub engine::TrainingConfig::seed: u64
impl engine::TrainingConfig
pub fn engine::TrainingConfig::with_batch_size(self, batch_size: usize) -> Self
pub fn engine::TrainingConfig::with_learning_rate(self, learning_rate: f64) -> Self
pub fn engine::TrainingConfig::with_num_epochs(self, num_epochs: usize) -> Self
pub fn engine::TrainingConfig::with_num_workers(self, num_workers: usize) -> Self
pub fn engine::TrainingConfig::with_seed(self, seed: u64) -> Self
impl burn_config::Config for engine::TrainingConfig
impl clone::Clone for engine::TrainingConfig
pub fn engine::TrainingConfig::clone(&self) -> Self
impl fmt::Display for engine::TrainingConfig
pub fn engine::TrainingConfig::fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
impl serde_ser::Serialize for engine::TrainingConfig
pub fn engine::TrainingConfig::serialize<S>(&self, serializer: S) -> result::Result<<S as serde_ser::Serializer>::Ok, <S as serde_ser::Serializer>::Error> where S: serde_ser::Serializer
impl<'de> serde_de::Deserialize<'de> for engine::TrainingConfig
pub fn engine::TrainingConfig::deserialize<D>(deserializer: D) -> result::Result<Self, <D as serde_de::Deserializer>::Error> where D: serde_de::Deserializer<'de>
pub fn engine::to_f32(game: &chess_game::ChessGame, from_sq: Option<&ChessSquare>, to_sq: Option<&ChessSquare>) -> ([f32; 896], [f32; 5])
pub fn engine::to_tensor<B: burn_backend::backend::base::Backend>(buffer: &alloc::vec::Vec<([f32; 896], [f32; 5])>, device: &<B as burn_backend::backend::base::Backend>::Device) -> (burn_tensor::tensor::api::base::Tensor<B, 3>, burn_tensor::tensor::api::base::Tensor<B, 2>)
pub struct ZobristKeys
pub ZobristKeys::castling: [u64; 16]
pub ZobristKeys::en_passant: [u64; 8]
pub ZobristKeys::pieces: [[[u64; 64]; 6]; 2]
pub ZobristKeys::side_to_move: u64
impl ZobristKeys
pub fn ZobristKeys::get() -> &'static Self
pub fn ZobristKeys::new() -> Self
#[repr(usize)] pub enum Color
pub Color::Black = 1
pub Color::White = 0
#[repr(usize)] pub enum PieceType
pub PieceType::Bishop = 2
pub PieceType::King = 5
pub PieceType::Knight = 1
pub PieceType::Pawn = 0
pub PieceType::Queen = 4
pub PieceType::Rook = 3
pub struct Bitboard(pub u64)
pub struct CastlingRights(pub u8)
pub struct ChessBoard
pub ChessBoard::all_pieces: Bitboard
pub ChessBoard::black_occupancy: Bitboard
pub ChessBoard::pieces: [[Bitboard; 6]; 2]
pub ChessBoard::white_occupancy: Bitboard
pub struct ChessGame
pub ChessGame::castling_rights: castling::CastlingRights
pub ChessGame::chessboard: chess_board::ChessBoard
pub ChessGame::en_passant: Option<ChessSquare>
pub ChessGame::fullmove_counter: u32
pub ChessGame::game_history: alloc::vec::Vec<chess_game::GameStateEntry>
pub ChessGame::halfmove_clock: u32
pub ChessGame::outcome: chess_game::Outcome
pub ChessGame::rule_set: chess_game::RuleSet
pub ChessGame::side_to_move: chess_piece::Color
pub ChessGame::zobrist_hash: u64
pub struct ChessMove
pub ChessMove::from: ChessSquare
pub ChessMove::promotion: Option<chess_piece::PieceType>
pub ChessMove::to: ChessSquare
pub struct ChessPiece
pub ChessPiece::color: chess_piece::Color
pub ChessPiece::piece_type: chess_piece::PieceType
pub struct ChessSquare(pub u8)
pub struct ChessTransformer<B: burn_backend::backend::base::Backend>
pub struct ReplayBuffer
pub ReplayBuffer::buffer: alloc::vec::Vec<TrainingSample>
pub ReplayBuffer::capacity: usize
pub ReplayBuffer::pointer: usize
pub struct ZobristKeys
pub ZobristKeys::castling: [u64; 16]
pub ZobristKeys::en_passant: [u64; 8]
pub ZobristKeys::pieces: [[[u64; 64]; 6]; 2]
pub ZobristKeys::side_to_move: u64

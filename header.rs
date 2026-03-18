pub use burn
pub struct Bitboard(pub u64)
impl Bitboard
pub const Bitboard::ALL: Bitboard
pub const Bitboard::ALL_PIECES: Bitboard
pub const Bitboard::BLACK_BISHOPS: Bitboard
pub const Bitboard::BLACK_KING: Bitboard
pub const Bitboard::BLACK_KNIGHTS: Bitboard
pub const Bitboard::BLACK_OCCUPANCY: Bitboard
pub const Bitboard::BLACK_PAWNS: Bitboard
pub const Bitboard::BLACK_QUEENS: Bitboard
pub const Bitboard::BLACK_ROOKS: Bitboard
pub const Bitboard::EMPTY: Bitboard
pub const Bitboard::WHITE_BISHOPS: Bitboard
pub const Bitboard::WHITE_KING: Bitboard
pub const Bitboard::WHITE_KNIGHTS: Bitboard
pub const Bitboard::WHITE_OCCUPANCY: Bitboard
pub const Bitboard::WHITE_PAWNS: Bitboard
pub const Bitboard::WHITE_QUEENS: Bitboard
pub const Bitboard::WHITE_ROOKS: Bitboard
pub fn Bitboard::clear(&mut self, square: ChessSquare)
pub fn Bitboard::count(&self) -> u32
pub fn Bitboard::flip(&mut self)
pub fn Bitboard::flipped(&self) -> Bitboard
pub fn Bitboard::from_square(square: ChessSquare) -> Self
pub fn Bitboard::is_empty(&self) -> bool
pub fn Bitboard::is_set(&self, square: ChessSquare) -> bool
pub fn Bitboard::lsb_square(&self) -> Option<ChessSquare>
pub fn Bitboard::msb_square(&self) -> Option<ChessSquare>
pub fn Bitboard::pop_lsb(&mut self) -> Option<ChessSquare>
pub fn Bitboard::pop_msb(&mut self) -> Option<ChessSquare>
pub const fn Bitboard::set(&mut self, square: ChessSquare)
pub fn Bitboard::shift_east(self) -> Self
pub fn Bitboard::shift_north(self) -> Self
pub fn Bitboard::shift_north_east(self) -> Self
pub fn Bitboard::shift_north_west(self) -> Self
pub fn Bitboard::shift_south(self) -> Self
pub fn Bitboard::shift_south_east(self) -> Self
pub fn Bitboard::shift_south_west(self) -> Self
pub fn Bitboard::shift_west(self) -> Self
pub fn Bitboard::to_bool(&self) -> [bool; 64]
pub fn Bitboard::toggle(&mut self, square: ChessSquare)
pub fn Bitboard::write_to_slice(&self, slice: &mut [f32])
impl fmt::Display for Bitboard
pub fn Bitboard::fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
impl ops::bit::BitAnd for Bitboard
pub type Bitboard::Output = Bitboard
pub fn Bitboard::bitand(self, rhs: Self) -> Self::Output
impl ops::bit::BitAnd<&Bitboard> for &Bitboard
pub type &Bitboard::Output = Bitboard
pub fn &Bitboard::bitand(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitAnd<&Bitboard> for Bitboard
pub fn Bitboard::bitand(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitAnd<Bitboard> for &Bitboard
pub fn &Bitboard::bitand(self, rhs: Bitboard) -> Self::Output
impl ops::bit::BitAndAssign for Bitboard
pub fn Bitboard::bitand_assign(&mut self, rhs: Self)
impl ops::bit::BitOr for Bitboard
pub fn Bitboard::bitor(self, rhs: Self) -> Self::Output
impl ops::bit::BitOr<&Bitboard> for &Bitboard
pub fn &Bitboard::bitor(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitOr<&Bitboard> for Bitboard
pub fn Bitboard::bitor(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitOr<Bitboard> for &Bitboard
pub fn &Bitboard::bitor(self, rhs: Bitboard) -> Self::Output
impl ops::bit::BitOrAssign for Bitboard
pub fn Bitboard::bitor_assign(&mut self, rhs: Self)
impl ops::bit::BitXor for Bitboard
pub fn Bitboard::bitxor(self, rhs: Self) -> Self::Output
impl ops::bit::BitXor<&Bitboard> for &Bitboard
pub fn &Bitboard::bitxor(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitXor<&Bitboard> for Bitboard
pub fn Bitboard::bitxor(self, rhs: &Bitboard) -> Self::Output
impl ops::bit::BitXor<Bitboard> for &Bitboard
pub fn &Bitboard::bitxor(self, rhs: Bitboard) -> Self::Output
impl ops::bit::BitXorAssign for Bitboard
pub fn Bitboard::bitxor_assign(&mut self, rhs: Self)
impl ops::bit::Not for Bitboard
pub fn Bitboard::not(self) -> Self::Output
pub struct CastlingRights(pub u8)
impl CastlingRights
pub const CastlingRights::BLACK_KINGSIDE: CastlingRights
pub const CastlingRights::BLACK_QUEENSIDE: CastlingRights
pub const CastlingRights::WHITE_KINGSIDE: CastlingRights
pub const CastlingRights::WHITE_QUEENSIDE: CastlingRights
pub fn CastlingRights::empty() -> Self
pub fn CastlingRights::flip_perspective(&self) -> CastlingRights
pub fn CastlingRights::from_fen(fen_part: &str) -> Self
pub fn CastlingRights::has(&self, right: CastlingRights) -> bool
pub fn CastlingRights::new() -> Self
pub fn CastlingRights::remove(&mut self, rights_to_remove: CastlingRights)
pub fn CastlingRights::to_fen(&self) -> alloc::string::String
impl ops::bit::BitAndAssign for CastlingRights
pub fn CastlingRights::bitand_assign(&mut self, rhs: Self)
impl ops::bit::BitOr for CastlingRights
pub type CastlingRights::Output = CastlingRights
pub fn CastlingRights::bitor(self, rhs: Self) -> Self::Output
impl ops::bit::BitOr<CastlingRights> for u8
pub type u8::Output = CastlingRights
pub fn u8::bitor(self, rhs: CastlingRights) -> Self::Output
impl ops::bit::BitOr<u8> for CastlingRights
pub fn CastlingRights::bitor(self, rhs: u8) -> Self::Output
impl ops::bit::BitOrAssign for CastlingRights
pub fn CastlingRights::bitor_assign(&mut self, rhs: Self)
impl ops::bit::BitXor for CastlingRights
pub fn CastlingRights::bitxor(self, rhs: Self) -> Self::Output
impl ops::bit::Not for CastlingRights
pub fn CastlingRights::not(self) -> Self::Output
pub struct ChessBoard
pub ChessBoard::all_pieces: Bitboard
pub ChessBoard::black_occupancy: Bitboard
pub ChessBoard::pieces: [[Bitboard; 6]; 2]
pub ChessBoard::white_occupancy: Bitboard
impl ChessBoard
pub const ChessBoard::BETWEEN: [[Option<Bitboard>; 64]; 64]
pub const ChessBoard::BISHOP_ATTACKS: [[Bitboard; 4]; 64]
pub const ChessBoard::BLACK_SQUARES: Bitboard
pub const ChessBoard::KING_ATTACKS: [Bitboard; 64]
pub const ChessBoard::KNIGHT_ATTACKS: [Bitboard; 64]
pub const ChessBoard::PAWN_ATTACKS_BLACK: [Bitboard; 64]
pub const ChessBoard::PAWN_ATTACKS_WHITE: [Bitboard; 64]
pub const ChessBoard::ROOK_ATTACKS: [[Bitboard; 4]; 64]
pub const ChessBoard::WHITE_SQUARES: Bitboard
pub fn ChessBoard::add_piece(&mut self, piece: chess_piece::ChessPiece, square: ChessSquare)
pub fn ChessBoard::apply_move(&mut self, mov: &chess_move::ChessMove, side_to_move: chess_piece::Color, en_passant_sq: Option<ChessSquare>)
pub fn ChessBoard::display_ascii(&self) -> alloc::string::String
pub fn ChessBoard::empty() -> Self
pub fn ChessBoard::flip_board(&self) -> Self
pub const fn ChessBoard::generate_rook_direction_masks() -> [[Bitboard; 4]; 64]
pub fn ChessBoard::get_piece_at(&self, square: ChessSquare) -> Option<chess_piece::ChessPiece>
pub fn ChessBoard::get_piece_bitboard(&self, color: chess_piece::Color, piece_type: chess_piece::PieceType) -> Bitboard
pub fn ChessBoard::is_square_attacked(&self, sq: ChessSquare, attacker_color: chess_piece::Color) -> bool
pub fn ChessBoard::move_piece(&mut self, from_sq: ChessSquare, to_sq: ChessSquare, piece: chess_piece::ChessPiece)
pub fn ChessBoard::new() -> Self
pub fn ChessBoard::remove_piece(&mut self, piece: chess_piece::ChessPiece, square: ChessSquare)
pub enum Outcome
pub Outcome::Finished(Option<chess_piece::Color>)
pub Outcome::Unfinished
pub enum RuleSet
pub RuleSet::Legal
pub RuleSet::PseudoLegal
pub struct ChessGame
pub ChessGame::castling_rights: CastlingRights
pub ChessGame::chessboard: ChessBoard
pub ChessGame::en_passant: Option<ChessSquare>
pub ChessGame::fullmove_counter: u32
pub ChessGame::game_history: alloc::vec::Vec<GameStateEntry>
pub ChessGame::halfmove_clock: u32
pub ChessGame::outcome: Outcome
pub ChessGame::rule_set: RuleSet
pub ChessGame::side_to_move: chess_piece::Color
pub ChessGame::zobrist_hash: u64
impl ChessGame
pub fn ChessGame::calculate_hash(&mut self) -> u64
pub fn ChessGame::check_game_state(&self) -> Outcome
pub fn ChessGame::fen_to_ascii(fen: &str)
pub fn ChessGame::from_fen(fen: &str) -> Self
pub fn ChessGame::generate_pseudolegal(&self) -> alloc::vec::Vec<chess_move::ChessMove>
pub fn ChessGame::is_legal(&self, mov: &chess_move::ChessMove) -> bool
pub fn ChessGame::make_move(&mut self, mov: &chess_move::ChessMove)
pub fn ChessGame::to_fen(&self) -> alloc::string::String
pub fn ChessGame::uci_to_move(&self, input: &str) -> result::Result<chess_move::ChessMove, &str>
impl default::Default for ChessGame
pub fn ChessGame::default() -> Self
pub struct GameStateEntry
pub GameStateEntry::castling_rights: CastlingRights
pub GameStateEntry::chessboard: ChessBoard
pub GameStateEntry::en_passant: Option<ChessSquare>
pub GameStateEntry::fullmove_counter: u32
pub GameStateEntry::halfmove_clock: u32
pub GameStateEntry::side_to_move: chess_piece::Color
pub GameStateEntry::zobrist_hash: u64
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
pub struct data::ChessBatch<B: burn_backend::backend::base::Backend>
pub data::ChessBatch::boards: burn_tensor::tensor::api::base::Tensor<B, 3>
pub data::ChessBatch::metas: burn_tensor::tensor::api::base::Tensor<B, 2>
pub data::ChessBatch::policy_targets: burn_tensor::tensor::api::base::Tensor<B, 2>
pub data::ChessBatch::value_targets: burn_tensor::tensor::api::base::Tensor<B, 2>
impl<B: burn_backend::backend::base::Backend> burn_data::dataloader::batcher::Batcher<B, data::TrainingSample, data::ChessBatch<B>> for data::ChessBatcher
pub fn data::ChessBatcher::batch(&self, items: alloc::vec::Vec<data::TrainingSample>, device: &<B as burn_backend::backend::base::Backend>::Device) -> data::ChessBatch<B>
pub struct data::ChessBatcher
pub struct data::ReplayBuffer
pub data::ReplayBuffer::buffer: alloc::vec::Vec<data::TrainingSample>
pub data::ReplayBuffer::capacity: usize
pub data::ReplayBuffer::pointer: usize
impl data::ReplayBuffer
pub fn data::ReplayBuffer::new(capacity: usize) -> Self
pub fn data::ReplayBuffer::push(&mut self, sample: &data::TrainingSample)
pub fn data::ReplayBuffer::sample_batch<B: burn_backend::backend::base::Backend>(&self, batch_size: usize, rng: &mut rand::rngs::small::SmallRng, device: &<B as burn_backend::backend::base::Backend>::Device) -> data::ChessBatch<B>
pub struct data::TrainingSample
pub data::TrainingSample::board: [f32; 896]
pub data::TrainingSample::meta: [f32; 5]
pub data::TrainingSample::target_policy: [f32; 64]
pub data::TrainingSample::target_value: [f32; 3]
impl default::Default for data::TrainingSample
pub fn data::TrainingSample::default() -> Self
pub struct TrainingConfig
pub TrainingConfig::batch_size: usize
pub TrainingConfig::learning_rate: f64
pub TrainingConfig::masked: bool
pub TrainingConfig::num_epochs: usize
pub TrainingConfig::num_workers: usize
pub TrainingConfig::optimizer: burn_optim::optim::adam::AdamConfig
pub TrainingConfig::seed: u64
impl TrainingConfig
pub fn TrainingConfig::with_batch_size(self, batch_size: usize) -> Self
pub fn TrainingConfig::with_learning_rate(self, learning_rate: f64) -> Self
pub fn TrainingConfig::with_num_epochs(self, num_epochs: usize) -> Self
pub fn TrainingConfig::with_num_workers(self, num_workers: usize) -> Self
pub fn TrainingConfig::with_seed(self, seed: u64) -> Self
impl burn_config::Config for TrainingConfig
impl clone::Clone for TrainingConfig
pub fn TrainingConfig::clone(&self) -> Self
impl fmt::Display for TrainingConfig
pub fn TrainingConfig::fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
pub fn to_f32(game: &ChessGame, from_sq: Option<&ChessSquare>, to_sq: Option<&ChessSquare>) -> ([f32; 896], [f32; 5])
pub fn to_tensor<B: burn_backend::backend::base::Backend>(buffer: &alloc::vec::Vec<([f32; 896], [f32; 5])>, device: &<B as burn_backend::backend::base::Backend>::Device) -> (burn_tensor::tensor::api::base::Tensor<B, 3>, burn_tensor::tensor::api::base::Tensor<B, 2>)
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
pub ChessGame::castling_rights: CastlingRights
pub ChessGame::chessboard: ChessBoard
pub ChessGame::en_passant: Option<ChessSquare>
pub ChessGame::fullmove_counter: u32
pub ChessGame::game_history: alloc::vec::Vec<GameStateEntry>
pub ChessGame::halfmove_clock: u32
pub ChessGame::outcome: Outcome
pub ChessGame::rule_set: RuleSet
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
pub ReplayBuffer::buffer: alloc::vec::Vec<data::TrainingSample>
pub ReplayBuffer::capacity: usize
pub ReplayBuffer::pointer: usize
pub struct ZobristKeys
pub ZobristKeys::castling: [u64; 16]
pub ZobristKeys::en_passant: [u64; 8]
pub ZobristKeys::pieces: [[[u64; 64]; 6]; 2]
pub ZobristKeys::side_to_move: u64

use core::fmt;

use arrayvec::ArrayVec;
use log::trace;

use crate::{Bitboard, CastlingRights, ChessBoard, ChessMove, ChessSquare, Color, PieceType, ZobristKeys, chess_game::Outcome};

#[derive(Debug, Clone, Default)]
pub struct ChessPosition {
    pub chessboard: ChessBoard,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<ChessSquare>,
    pub halfmove_clock: u32,
    pub zobrist_hash: u64,
    pub pseudolegal_moves: ArrayVec<ChessMove, 128>,
}

impl fmt::Display for ChessPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let moves: String = self.pseudolegal_moves.iter().map(|mov| mov.to_uci() + " ").collect();
        write!(f, "Side to move: {}\n{}\npseudolegal moves: {}", self.side_to_move, self.chessboard.display_ascii(), moves)
    }
}

impl ChessPosition {
    pub fn generate_pseudolegal(&mut self) {
        let mut moves = ArrayVec::<ChessMove, 128>::new();

        let (allies, opps) = match self.side_to_move {
            Color::White => (self.chessboard.white_occupancy, self.chessboard.black_occupancy),
            Color::Black => (self.chessboard.black_occupancy, self.chessboard.white_occupancy),
        };

        let mut pawns = self.chessboard.pieces[self.side_to_move as usize][PieceType::Pawn as usize];
        let mut knights = self.chessboard.pieces[self.side_to_move as usize][PieceType::Knight as usize];
        let mut bishops = self.chessboard.pieces[self.side_to_move as usize][PieceType::Bishop as usize];
        let mut rooks = self.chessboard.pieces[self.side_to_move as usize][PieceType::Rook as usize];
        let mut queens = self.chessboard.pieces[self.side_to_move as usize][PieceType::Queen as usize];
        let mut king = self.chessboard.pieces[self.side_to_move as usize][PieceType::King as usize];

        while let Some(from_sq) = pawns.pop_lsb() {
            let side = self.side_to_move;
            let rank_7 = if side == Color::White { 6 } else { 1 };
            let rank_2 = if side == Color::White { 1 } else { 6 };

            let add_move = |moves2: &mut ArrayVec<ChessMove, 128>, from: ChessSquare, to: ChessSquare| {
                if from.rank() == rank_7 {
                    for piece in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                        let _ = moves2.try_push(ChessMove::new(from, to, Some(piece)));
                    }
                } else {
                    let _ = moves2.try_push(ChessMove::new(from, to, None));
                }
            };

            let square_ahead = if self.side_to_move == Color::White {
                from_sq.square_north()
            } else {
                from_sq.square_south()
            };

            if let Some(to_sq) = square_ahead {
                if !self.chessboard.all_pieces.is_set(to_sq) {
                    add_move(&mut moves, from_sq, to_sq);
                    if from_sq.rank() == rank_2 {
                        let square_ahead = if self.side_to_move == Color::White {
                            to_sq.square_north()
                        } else {
                            to_sq.square_south()
                        };
                        if let Some(to_sq) = square_ahead {
                            if !self.chessboard.all_pieces.is_set(to_sq) {
                                let _ = moves.try_push(ChessMove::new(from_sq, to_sq, None));
                            }
                        }
                    }
                }
            }
            // Captures
            let mut attacks = if side == Color::White {
                ChessBoard::PAWN_ATTACKS_WHITE[from_sq.0 as usize]
            } else {
                ChessBoard::PAWN_ATTACKS_BLACK[from_sq.0 as usize]
            };

            let mut targets = opps;
            if let Some(ep_sq) = self.en_passant {
                targets |= Bitboard::from_square(ep_sq);
            }

            attacks &= targets;

            while let Some(to_sq) = attacks.pop_lsb() {
                add_move(&mut moves, from_sq, to_sq);
            }
        }

        while let Some(from_sq) = knights.pop_lsb() {
            let mut to_squares = ChessBoard::KNIGHT_ATTACKS[from_sq.0 as usize] & !allies;
            while let Some(to_sq) = to_squares.pop_lsb() {
                let mv = ChessMove::new(from_sq, to_sq, None);
                let _ = moves.try_push(mv);
            }
        }

        let mut move_pusher = |from_sq: ChessSquare, ray_bb: [Bitboard; 4]| {
            let mut ray = Bitboard::EMPTY;
            for i in 0..4 {
                let mut blockers = ray_bb[i] & self.chessboard.all_pieces;
                if blockers.is_empty() {
                    ray |= ray_bb[i]
                } else {
                    let to_sq = if i < 2 { blockers.pop_msb() } else { blockers.pop_lsb() };
                    let to_sq = to_sq.expect("No to_sq found in blockers");
                    if opps.is_set(to_sq) {
                        ray.set(to_sq);
                    }
                    // for some reason, i made all empty bitboards None, when adjacent squares
                    // should be empty instead, so i have to use unwrap or default
                    ray |= ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].unwrap_or_default()
                };
            }
            while let Some(to_sq) = ray.pop_lsb() {
                let _ = moves.try_push(ChessMove::new(from_sq, to_sq, None));
            }
        };

        while let Some(from_sq) = bishops.pop_lsb() {
            let ray_bb = ChessBoard::BISHOP_ATTACKS[from_sq.0 as usize];
            move_pusher(from_sq, ray_bb);
        }

        while let Some(from_sq) = rooks.pop_lsb() {
            let ray_bb = ChessBoard::ROOK_ATTACKS[from_sq.0 as usize];
            move_pusher(from_sq, ray_bb);
        }

        while let Some(from_sq) = queens.pop_lsb() {
            let rooks = ChessBoard::ROOK_ATTACKS[from_sq.0 as usize];
            let bishops = ChessBoard::BISHOP_ATTACKS[from_sq.0 as usize];
            move_pusher(from_sq, rooks);
            move_pusher(from_sq, bishops);
        }

        if let Some(from_sq) = king.pop_lsb() {
            let mut bb = ChessBoard::KING_ATTACKS[from_sq.0 as usize] & !allies;
            while let Some(sq) = bb.pop_lsb() {
                let _ = moves.try_push(ChessMove::new(from_sq, sq, None));
            }
            let clear = |from: ChessSquare, to: ChessSquare| -> bool {
                let mut between = ChessBoard::BETWEEN[from.0 as usize][to.0 as usize].expect("failed to find between sq castling");
                // If no blockers
                if (between & self.chessboard.all_pieces).is_empty() {
                    // If no squares in check (castling into check is pseudo legal, but not
                    // out of or through check)
                    let sq = between.pop_lsb().unwrap();
                    if !(self.chessboard.is_square_attacked(from, self.side_to_move.opposite())
                        || self.chessboard.is_square_attacked(sq, self.side_to_move.opposite()))
                    {
                        return true;
                    }
                }
                false
            };
            match self.side_to_move {
                Color::White => {
                    if self.castling_rights.has(CastlingRights::WHITE_KINGSIDE) {
                        if clear(ChessSquare::E1, ChessSquare::G1) {
                            let _ = moves.try_push(ChessMove::new(ChessSquare::E1, ChessSquare::G1, None));
                        }
                    }
                    if self.castling_rights.has(CastlingRights::WHITE_QUEENSIDE) {
                        if !self.chessboard.all_pieces.is_set(ChessSquare::B1) {
                            if clear(ChessSquare::E1, ChessSquare::C1) {
                                let _ = moves.try_push(ChessMove::new(ChessSquare::E1, ChessSquare::C1, None));
                            }
                        }
                    }
                }
                Color::Black => {
                    if self.castling_rights.has(CastlingRights::BLACK_KINGSIDE) {
                        if clear(ChessSquare::E8, ChessSquare::G8) {
                            let _ = moves.try_push(ChessMove::new(ChessSquare::E8, ChessSquare::G8, None));
                        }
                    }
                    if self.castling_rights.has(CastlingRights::BLACK_QUEENSIDE) {
                        if !self.chessboard.all_pieces.is_set(ChessSquare::B8) {
                            if clear(ChessSquare::E8, ChessSquare::C8) {
                                let _ = moves.try_push(ChessMove::new(ChessSquare::E8, ChessSquare::C8, None));
                            }
                        }
                    }
                }
            }
        }

        self.pseudolegal_moves = moves;
    }

    pub fn make_mask(&self, legal: bool, from_sq: Option<ChessSquare>) -> [bool; 64] {
        let mut mask = [false; 64];
        if let Some(from_sq) = from_sq {
            assert!(!self.pseudolegal_moves.is_empty());
            self.pseudolegal_moves.iter().for_each(|&mov| {
                if !legal || self.is_legal(&mov) {
                    if from_sq == mov.from {
                        mask[mov.to.0 as usize] = true;
                    }
                }
            });
        } else {
            assert!(!self.pseudolegal_moves.is_empty());
            self.pseudolegal_moves.iter().for_each(|&mov| {
                if !legal || self.is_legal(&mov) {
                    mask[mov.from.0 as usize] = true;
                }
            });
        }
        mask
    }

    pub fn is_legal(&self, mov: &ChessMove) -> bool {
        let mut temp_board = self.chessboard.clone();
        trace!("is_legal: chessboard: {}", self.chessboard.display_ascii());
        trace!("is_legal: checking move: {}", mov.to_uci());
        temp_board.apply_move(&mov, self.side_to_move, self.en_passant);

        let king_bb = temp_board.get_piece_bitboard(self.side_to_move, PieceType::King);
        let king_sq = king_bb.msb_square().unwrap();

        if temp_board.is_square_attacked(king_sq, self.side_to_move.opposite()) {
            return false;
        } else {
            return true;
        }
    }

    pub fn make_move(&mut self, mov: &ChessMove) {
        let moving_piece = self.chessboard.get_piece_at(mov.from).expect(&format!("No piece at from sq {}\n{}", mov.from, self));
        let captured_piece = self.chessboard.get_piece_at(mov.to);

        let mut rights_to_remove = CastlingRights::empty();
        if moving_piece.piece_type == PieceType::King {
            match self.side_to_move {
                Color::White => rights_to_remove |= CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE,
                Color::Black => rights_to_remove |= CastlingRights::BLACK_KINGSIDE | CastlingRights::BLACK_QUEENSIDE,
            }
        }

        let get_rights = |sq: ChessSquare| -> CastlingRights {
            match sq {
                ChessSquare::H1 => CastlingRights::WHITE_KINGSIDE,
                ChessSquare::A1 => CastlingRights::WHITE_QUEENSIDE,
                ChessSquare::H8 => CastlingRights::BLACK_KINGSIDE,
                ChessSquare::A8 => CastlingRights::BLACK_QUEENSIDE,
                _ => CastlingRights::empty(),
            }
        };

        rights_to_remove |= get_rights(mov.from);
        rights_to_remove |= get_rights(mov.to);
        self.castling_rights.remove(rights_to_remove);

        self.chessboard.apply_move(mov, self.side_to_move, self.en_passant);

        self.en_passant = None;
        if moving_piece.piece_type == PieceType::Pawn && (mov.from.rank() as i8 - mov.to.rank() as i8).abs() == 2 {
            let skipped_rank = (mov.from.rank() + mov.to.rank()) / 2;
            self.en_passant = ChessSquare::from_coords(mov.from.file(), skipped_rank);
        }

        if moving_piece.piece_type == PieceType::Pawn || captured_piece.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.side_to_move = self.side_to_move.opposite();

        self.generate_pseudolegal();

        self.zobrist_hash = self.calculate_hash();
    }

    pub fn calculate_hash(&self) -> u64 {
        let mut hash = 0;
        let keys = ZobristKeys::get();

        for color in [Color::White, Color::Black] {
            for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
                let mut bb = self.chessboard.get_piece_bitboard(color, piece_type);
                while let Some(sq) = bb.pop_lsb() {
                    hash ^= keys.pieces[color as usize][piece_type as usize][sq.0 as usize];
                }
            }
        }

        hash ^= keys.castling[self.castling_rights.0 as usize];

        if let Some(sq) = self.en_passant {
            hash ^= keys.en_passant[sq.file() as usize];
        }

        if self.side_to_move == Color::Black {
            hash ^= keys.side_to_move;
        }

        hash
    }

    pub fn check_game_state(&self, legal: bool) -> Outcome {
        // pseudolegal checks
        if self.halfmove_clock >= 80 {
            return Outcome::Finished(None);
        }
        if self.chessboard.get_piece_bitboard(Color::White, PieceType::King).is_empty() {
            return Outcome::Finished(Some(Color::Black));
        }
        if self.chessboard.get_piece_bitboard(Color::Black, PieceType::King).is_empty() {
            return Outcome::Finished(Some(Color::White));
        }

        // if !legal {
        //     return Outcome::Unfinished;
        // }
        //
        // Legal Checks
        let mut king_bb = self.chessboard.get_piece_bitboard(self.side_to_move, PieceType::King);
        let king_sq = king_bb.pop_lsb().unwrap();

        if legal {
            if !self.pseudolegal_moves.iter().any(|&mov| self.is_legal(&mov)) {
                if self.chessboard.is_square_attacked(king_sq, self.side_to_move.opposite()) {
                    return Outcome::Finished(Some(self.side_to_move.opposite()));
                } else {
                    return Outcome::Finished(None);
                }
            }
        }

        // insufficient material
        let all_pieces = self.chessboard.all_pieces;
        let count = all_pieces.count();

        if count == 2 {
            return Outcome::Finished(None);
        }
        let mut white_bishops = self.chessboard.get_piece_bitboard(Color::White, PieceType::Bishop);
        let white_knights = self.chessboard.get_piece_bitboard(Color::White, PieceType::Knight);
        let mut black_bishops = self.chessboard.get_piece_bitboard(Color::Black, PieceType::Bishop);
        let black_knights = self.chessboard.get_piece_bitboard(Color::Black, PieceType::Knight);

        let white_minors = white_bishops | white_knights;
        let black_minors = black_bishops | black_knights;

        if count == 3 {
            if !white_minors.is_empty() || !black_minors.is_empty() {
                return Outcome::Finished(None);
            }
        }

        if count == 4 {
            // K + N vs K + N or K + N + N vs K
            if white_bishops.is_empty() && black_bishops.is_empty() {
                return Outcome::Finished(None);
            }

            if white_bishops.count() == 1 && black_bishops.count() == 1 {
                let w_sq = white_bishops.pop_lsb().unwrap();
                let b_sq = black_bishops.pop_lsb().unwrap();
                if w_sq.colour() == b_sq.colour() {
                    return Outcome::Finished(None);
                }
            }

            if black_bishops.count() == 2 {
                if let (Some(sq1), Some(sq2)) = (black_bishops.pop_msb(), black_bishops.pop_msb()) {
                    if sq1.colour() == sq2.colour() {
                        return Outcome::Finished(None);
                    }
                }
            }
            if white_bishops.count() == 2 {
                if let (Some(sq1), Some(sq2)) = (white_bishops.pop_msb(), white_bishops.pop_msb()) {
                    if sq1.colour() == sq2.colour() {
                        return Outcome::Finished(None);
                    }
                }
            }
        }

        Outcome::Unfinished
    }

    pub fn expand_if_prom(&self, mov: ChessMove) -> Option<[ChessMove; 4]> {
        let prom_rank = match self.side_to_move {
            Color::White => 7,
            Color::Black => 0,
        };
        if let Some(piece) = self.chessboard.get_piece_at(mov.from)
            && matches!(piece.piece_type, PieceType::Pawn)
            && mov.to.rank() == prom_rank
        {
            return Some([
                mov.with_prom(PieceType::Knight),
                mov.with_prom(PieceType::Bishop),
                mov.with_prom(PieceType::Rook),
                mov.with_prom(PieceType::Queen),
            ]);
        }
        None
    }
}

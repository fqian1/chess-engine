use crate::{
    Bitboard, CastlingRights, ChessBoard, ChessMove, ChessSquare, Color, PieceType, ZobristKeys, chess_game::Outcome,
};

#[derive(Debug, Clone, Default)]
pub struct ChessPosition {
    pub chessboard: ChessBoard,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<ChessSquare>,
    pub halfmove_clock: u32,
    pub zobrist_hash: u64,
    pub pseudolegal_moves: Vec<ChessMove>,
}

impl ChessPosition {
    pub fn generate_pseudolegal(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();

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

            let add_move = |moves2: &mut Vec<ChessMove>, from: ChessSquare, to: ChessSquare| {
                if from.rank() == rank_7 {
                    for piece in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                        moves2.push(ChessMove::new(from, to, Some(piece)));
                    }
                } else {
                    moves2.push(ChessMove::new(from, to, None));
                }
            };

            let square_ahead = if self.side_to_move == Color::White {
                from_sq.square_north()
            } else {
                from_sq.square_south()
            };

            if let Some(to_sq) = square_ahead {
                if !self.chessboard.all_pieces.is_set(to_sq) {
                    if cfg!(debug_assertions) {
                        println!("Single push: {}", to_sq);
                    }
                    add_move(&mut moves, from_sq, to_sq);
                    if from_sq.rank() == rank_2 {
                        let square_ahead = if self.side_to_move == Color::White {
                            to_sq.square_north()
                        } else {
                            to_sq.square_south()
                        };
                        if let Some(to_sq) = square_ahead {
                            if !self.chessboard.all_pieces.is_set(to_sq) {
                                if cfg!(debug_assertions) {
                                    println!("Double push: {}", to_sq);
                                }
                                moves.push(ChessMove::new(from_sq, to_sq, None));
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
                moves.push(mv);
            }
        }

        let mut move_pusher = |from_sq: ChessSquare, ray_bb: [Bitboard; 4]| {
            let mut ray = Bitboard::EMPTY;
            for i in 0..4 {
                let mut blockers = ray_bb[i] & self.chessboard.all_pieces;
                if cfg!(debug_assertions) {
                    // println!("from_sq: {}\nray: {}\nblocker: {}", from_sq, ray_bb[i], blockers);
                    // println!("blockers: {}\n all_pieces: {}",blockers, self.chessboard.all_pieces);
                }
                if blockers.is_empty() {
                    ray |= ray_bb[i]
                } else {
                    let to_sq = if i < 2 { blockers.pop_msb() } else { blockers.pop_lsb() };
                    let to_sq = to_sq.expect("No to_sq found in blockers");
                    if opps.is_set(to_sq) {
                        if cfg!(debug_assertions) {
                            println!(
                                "from_sq: {}\nblockers:\n{}\nray:\n {}\nto_sq: {}",
                                from_sq, ray_bb[i], blockers, to_sq
                            );
                        }
                        ray.set(to_sq);
                    }
                    // for some reason, i made all empty bitboards None, when adjacent squares
                    // should be empty instead, so i have to use unwrap or default
                    ray |= ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].unwrap_or_default()
                };
            }
            while let Some(to_sq) = ray.pop_lsb() {
                moves.push(ChessMove::new(from_sq, to_sq, None));
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
                moves.push(ChessMove::new(from_sq, sq, None));
            }
            let clear = |from: ChessSquare, to: ChessSquare| -> bool {
                let mut between =
                    ChessBoard::BETWEEN[from.0 as usize][to.0 as usize].expect("failed to find between sq castling");
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
                            moves.push(ChessMove::new(ChessSquare::E1, ChessSquare::G1, None));
                        }
                    }
                    if self.castling_rights.has(CastlingRights::WHITE_QUEENSIDE) {
                        if !self.chessboard.all_pieces.is_set(ChessSquare::B1) {
                            if clear(ChessSquare::E1, ChessSquare::C1) {
                                moves.push(ChessMove::new(ChessSquare::E1, ChessSquare::C1, None));
                            }
                        }
                    }
                }
                Color::Black => {
                    if self.castling_rights.has(CastlingRights::BLACK_KINGSIDE) {
                        if clear(ChessSquare::E8, ChessSquare::G8) {
                            moves.push(ChessMove::new(ChessSquare::E8, ChessSquare::G8, None));
                        }
                    }
                    if self.castling_rights.has(CastlingRights::BLACK_QUEENSIDE) {
                        if !self.chessboard.all_pieces.is_set(ChessSquare::B8) {
                            if clear(ChessSquare::E8, ChessSquare::C8) {
                                moves.push(ChessMove::new(ChessSquare::E8, ChessSquare::C8, None));
                            }
                        }
                    }
                }
            }
        }
        moves
    }

    pub fn make_mask(&self, legal: bool, from_sq: Option<ChessSquare>) -> [bool; 64] {
        match from_sq {
            Some(sq) => std::array::from_fn::<bool, 64, _>(|i| {
                self.pseudolegal_moves
                    .iter()
                    .filter(|mov| !legal || self.is_legal(mov))
                    .any(|mov| mov.from == sq && mov.to.0 == i as u8)
            }),
            None => std::array::from_fn::<bool, 64, _>(|i| {
                self.pseudolegal_moves
                    .iter()
                    .filter(|mov| !legal || self.is_legal(mov))
                    .any(|mov| mov.from.0 == i as u8)
            }),
        }
    }

    pub fn is_legal(&self, mov: &ChessMove) -> bool {
        let mut temp_board = self.chessboard.clone();
        temp_board.apply_move(&mov, self.side_to_move, self.en_passant);

        let mut king_bb = temp_board.get_piece_bitboard(self.side_to_move, PieceType::King);
        let king_sq = king_bb.pop_lsb().expect("is_legal says: no king?");

        if temp_board.is_square_attacked(king_sq, self.side_to_move.opposite()) {
            return false;
        } else {
            return true;
        }
    }

    pub fn make_move(&mut self, mov: &ChessMove) {
        let keys = ZobristKeys::get();
        let moving_piece = self.chessboard.get_piece_at(mov.from).expect("No piece at from sq");
        let captured_piece = self.chessboard.get_piece_at(mov.to);

        let is_en_passant =
            moving_piece.piece_type == PieceType::Pawn && self.en_passant.is_some_and(|sq| sq == mov.to);
        let is_castling =
            moving_piece.piece_type == PieceType::King && (mov.from.file() as i8 - mov.to.file() as i8).abs() == 2;

        // Remove Old Global State from Hash
        self.zobrist_hash ^= keys.castling[self.castling_rights.0 as usize];
        if let Some(ep) = self.en_passant {
            self.zobrist_hash ^= keys.en_passant[ep.file() as usize];
        }
        self.zobrist_hash ^= keys.side_to_move;

        // Remove Moving Piece from From
        self.zobrist_hash ^=
            keys.pieces[moving_piece.color as usize][moving_piece.piece_type as usize][mov.from.0 as usize];

        // Remove Captured Piece
        if is_en_passant {
            let cap_sq_idx = if self.side_to_move == Color::White {
                mov.to.0 - 8
            } else {
                mov.to.0 + 8
            };
            self.zobrist_hash ^=
                keys.pieces[self.side_to_move.opposite() as usize][PieceType::Pawn as usize][cap_sq_idx as usize];
        } else if let Some(cap) = captured_piece {
            self.zobrist_hash ^= keys.pieces[cap.color as usize][cap.piece_type as usize][mov.to.0 as usize];
        }

        // Update Hash
        // Add Moving Piece to Destination
        let final_piece_type = mov.promotion.unwrap_or(moving_piece.piece_type);
        self.zobrist_hash ^= keys.pieces[moving_piece.color as usize][final_piece_type as usize][mov.to.0 as usize];

        // Handle Castling Rook
        if is_castling {
            let (rook_from, rook_to) = match (self.side_to_move, mov.to.file()) {
                (Color::White, 6) => (ChessSquare::H1, ChessSquare::F1),
                (Color::White, 2) => (ChessSquare::A1, ChessSquare::D1),
                (Color::Black, 6) => (ChessSquare::H8, ChessSquare::F8),
                (Color::Black, 2) => (ChessSquare::A8, ChessSquare::D8),
                _ => unreachable!(),
            };
            // Remove Rook from corner
            self.zobrist_hash ^=
                keys.pieces[self.side_to_move as usize][PieceType::Rook as usize][rook_from.0 as usize];
            // Add Rook to new square
            self.zobrist_hash ^= keys.pieces[self.side_to_move as usize][PieceType::Rook as usize][rook_to.0 as usize];
        }

        // Update Internal Game State
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

        // Update En Passant
        self.en_passant = None;
        if moving_piece.piece_type == PieceType::Pawn && (mov.from.rank() as i8 - mov.to.rank() as i8).abs() == 2 {
            let skipped_rank = (mov.from.rank() + mov.to.rank()) / 2;
            self.en_passant = ChessSquare::from_coords(mov.from.file(), skipped_rank);
        }

        // Update Clocks
        if moving_piece.piece_type == PieceType::Pawn || captured_piece.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.chessboard.apply_move(mov, self.side_to_move, self.en_passant);

        self.side_to_move = self.side_to_move.opposite();

        // Hash global state
        self.zobrist_hash ^= keys.castling[self.castling_rights.0 as usize];
        if let Some(ep) = self.en_passant {
            self.zobrist_hash ^= keys.en_passant[ep.file() as usize];
        }

        debug_assert!(self.zobrist_hash == self.calculate_hash());
    }

    pub fn calculate_hash(&self) -> u64 {
        let mut hash = 0;
        let keys = ZobristKeys::get();

        for sq_idx in 0..64 {
            let sq = ChessSquare(sq_idx);
            if let Some(piece) = self.chessboard.get_piece_at(sq) {
                hash ^= keys.pieces[piece.color as usize][piece.piece_type as usize][sq_idx as usize];
            }
        }

        for color in [Color::White, Color::Black] {
            for piece_type in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ] {
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
        // PseudoLegal and Legal Checks
        if self.halfmove_clock >= 100 {
            return Outcome::Finished(None);
        }

        // King capture
        if self.chessboard.get_piece_bitboard(Color::White, PieceType::King).is_empty() {
            return Outcome::Finished(Some(Color::Black));
        }
        if self.chessboard.get_piece_bitboard(Color::Black, PieceType::King).is_empty() {
            return Outcome::Finished(Some(Color::White));
        }

        if legal {
            return Outcome::Unfinished;
        }

        // Legal Checks
        // checkmate
        let mut king_bb = self.chessboard.get_piece_bitboard(self.side_to_move, PieceType::King);
        // Safe, king must exist otherwise wouldve returned earlier
        let king_sq = king_bb.pop_lsb().unwrap();
        let mut legal_moves = self.generate_pseudolegal();
        legal_moves.retain(|x| self.is_legal(x));

        if legal_moves.is_empty() {
            if self.chessboard.is_square_attacked(king_sq, self.side_to_move.opposite()) {
                return Outcome::Finished(Some(self.side_to_move.opposite()));
            } else {
                return Outcome::Finished(None);
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
            // K + N vs K + N
            if white_bishops.is_empty() && black_bishops.is_empty() {
                return Outcome::Finished(None);
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

    pub fn is_geometrically_valid(&self, mov: &ChessMove) -> bool {
        let from_sq = mov.from;
        let to_sq = mov.to;
        let opponent = self.side_to_move.opposite();

        let Some(from_piece) = self.chessboard.get_piece_at(from_sq) else {
            return false;
        };

        if from_piece.color != self.side_to_move {
            return false;
        }

        if let Some(to_piece) = self.chessboard.get_piece_at(to_sq) {
            if to_piece.color == self.side_to_move {
                return false;
            }
        }

        match from_piece.piece_type {
            PieceType::Pawn => {
                let (direction, start_rank) = match self.side_to_move {
                    Color::White => (1, 1),
                    Color::Black => (-1, 6),
                };

                let rank_diff = to_sq.rank() as i8 - from_sq.rank() as i8;
                let file_diff = (to_sq.file() as i8 - from_sq.file() as i8).abs();

                match self.side_to_move {
                    Color::White => {
                        if mov.to.rank() != 7 && mov.promotion.is_some() {
                            return false;
                        }
                        if mov.to.rank() == 7
                            && (mov.promotion.is_none() || mov.promotion.is_some_and(|x| x == PieceType::Pawn))
                        {
                            return false;
                        }
                    }
                    Color::Black => {
                        if mov.to.rank() != 0 && mov.promotion.is_some() {
                            return false;
                        }
                        if mov.to.rank() == 0
                            && (mov.promotion.is_none() || mov.promotion.is_some_and(|x| x == PieceType::Pawn))
                        {
                            return false;
                        }
                    }
                }

                if file_diff == 0 {
                    if rank_diff == direction {
                        if self.chessboard.all_pieces.is_set(to_sq) {
                            return false;
                        }
                    } else if rank_diff == 2 * direction {
                        if from_sq.rank() != start_rank {
                            return false;
                        }
                        let mid_sq = ChessSquare((from_sq.0 as i8 + (8 * direction)) as u8);
                        if self.chessboard.all_pieces.is_set(to_sq) || self.chessboard.all_pieces.is_set(mid_sq) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else if file_diff == 1 {
                    if rank_diff != direction {
                        return false;
                    }
                    let is_ep = self.en_passant.is_some_and(|sq| sq == to_sq);

                    if !is_ep {
                        let target_occupancy = match self.side_to_move {
                            Color::White => self.chessboard.black_occupancy,
                            Color::Black => self.chessboard.white_occupancy,
                        };

                        if !target_occupancy.is_set(to_sq) {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }

            PieceType::Rook => {
                let rook_attacks = ChessBoard::ROOK_ATTACKS[from_sq.0 as usize]
                    .iter()
                    .copied()
                    .reduce(|acc, bb| acc | bb)
                    .unwrap_or(Bitboard::EMPTY);
                if !rook_attacks.is_set(to_sq) {
                    return false;
                }
                let ray_board = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].unwrap();

                if !(ray_board & self.chessboard.all_pieces).is_empty() {
                    return false;
                }
            }

            PieceType::Bishop => {
                let bishop_attacks = ChessBoard::BISHOP_ATTACKS[from_sq.0 as usize]
                    .iter()
                    .copied()
                    .reduce(|acc, bb| acc | bb)
                    .unwrap_or(Bitboard::EMPTY);
                if !bishop_attacks.is_set(to_sq) {
                    return false;
                }
                let ray_board = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].unwrap();

                if !(ray_board & self.chessboard.all_pieces).is_empty() {
                    return false;
                }
            }

            PieceType::Knight => {
                if !ChessBoard::KNIGHT_ATTACKS[from_sq.0 as usize].is_set(to_sq) {
                    return false;
                }
            }

            PieceType::Queen => {
                let bishop_attacks = ChessBoard::BISHOP_ATTACKS[from_sq.0 as usize]
                    .iter()
                    .copied()
                    .reduce(|acc, bb| acc | bb)
                    .unwrap_or(Bitboard::EMPTY);
                let rook_attacks = ChessBoard::ROOK_ATTACKS[from_sq.0 as usize]
                    .iter()
                    .copied()
                    .reduce(|acc, bb| acc | bb)
                    .unwrap_or(Bitboard::EMPTY);

                let is_diag = bishop_attacks.is_set(to_sq);
                let is_orth = rook_attacks.is_set(to_sq);

                if !is_diag && !is_orth {
                    return false;
                }

                let ray_board = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].unwrap();

                if !(ray_board & self.chessboard.all_pieces).is_empty() {
                    return false;
                }
            }

            PieceType::King => {
                let between = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].unwrap();

                if self.chessboard.all_pieces.is_set(to_sq) {
                    return false;
                }

                if !(self.chessboard.all_pieces & between).is_empty() {
                    return false;
                }

                if to_sq.file() == 1 {
                    let b_file_sq = if to_sq.rank() == 0 {
                        ChessSquare::B1
                    } else {
                        ChessSquare::B8
                    };
                    if self.chessboard.all_pieces.is_set(b_file_sq) {
                        return false;
                    }
                }

                let (rights_needed, crossing_sq) = match to_sq {
                    ChessSquare::G1 => (CastlingRights::WHITE_KINGSIDE, ChessSquare::F1),
                    ChessSquare::C1 => (CastlingRights::WHITE_QUEENSIDE, ChessSquare::D1),
                    ChessSquare::G8 => (CastlingRights::BLACK_KINGSIDE, ChessSquare::F8),
                    ChessSquare::C8 => (CastlingRights::BLACK_QUEENSIDE, ChessSquare::D8),
                    _ => return false,
                };

                if !self.castling_rights.has(rights_needed) {
                    return false;
                }

                if self.chessboard.is_square_attacked(from_sq, opponent) {
                    return false;
                }
                if self.chessboard.is_square_attacked(crossing_sq, opponent) {
                    return false;
                }

                if !ChessBoard::KING_ATTACKS[from_sq.0 as usize].is_set(to_sq) {
                    return false;
                }
            }
        }
        true
    }
}

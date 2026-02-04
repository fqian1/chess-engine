use super::{CastlingRights, ChessBoard, ChessMove, ChessPiece, ChessSquare, Color, PieceType, ZobristKeys};

pub enum MoveValidity {
    Valid,
    PseudoLegal(String), // Leaves king in check
    Impossible(String),  // Geometry violation
}

pub enum Outcome {
    Unfinished,
    Finished(Option<Color>),
}

#[derive(Debug, Clone)]
pub enum RuleSet {
    Legal,
    PseudoLegal,
}

#[derive(Debug, Clone)]
pub struct GameStateEntry {
    pub move_made: ChessMove,
    pub side_to_move: Color,
    pub captured_piece: Option<ChessPiece>,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<ChessSquare>,
    pub halfmove_clock: u32,
    pub fullmove_counter: u32,
    pub zobrist_hash: u64,
    pub rule_set: RuleSet,
}

#[derive(Debug, Clone)]
pub struct ChessGame {
    pub chessboard: ChessBoard,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<ChessSquare>,
    pub halfmove_clock: u32,
    pub fullmove_counter: u32,
    pub game_history: Vec<GameStateEntry>,
    pub zobrist_hash: u64,
    pub rule_set: RuleSet,
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl ChessGame {
    pub fn from_fen(fen: &str) -> Self {
        let mut parts = fen.split(' ');
        let board_str = parts.next().expect("FEN missing board");
        let side_str = parts.next().expect("FEN missing side to move");
        let castling_str = parts.next().expect("FEN missing castling rights");
        let ep_str = parts.next().expect("FEN missing en passant square");
        let halfmove_clock: u32 =
            parts.next().expect("FEN missing halfmove clock").parse().expect("Invalid halfmove clock");
        let fullmove_counter: u32 =
            parts.next().expect("FEN missing fullmove counter").parse().expect("Invalid fullmove counter");

        let mut board_array = [None; 64];

        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        for c in board_str.chars() {
            match c {
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => {
                    file += c.to_digit(10).unwrap() as u8;
                }
                _ => {
                    let color = if c.is_uppercase() { Color::White } else { Color::Black };

                    let piece_type = match c {
                        'P' | 'p' => PieceType::Pawn,
                        'N' | 'n' => PieceType::Knight,
                        'B' | 'b' => PieceType::Bishop,
                        'R' | 'r' => PieceType::Rook,
                        'Q' | 'q' => PieceType::Queen,
                        'K' | 'k' => PieceType::King,
                        _ => unreachable!("Invalid piece char"),
                    };

                    let index = (rank as usize) * 8 + (file as usize);

                    board_array[index] = Some(ChessPiece::new(color, piece_type));
                    file += 1;
                }
            }
        }

        let en_passant = match ep_str {
            "-" => None,
            s => Some(ChessSquare::from_name(s).expect("Invalid en passant square")),
        };

        let mut board = ChessBoard::empty();
        for (index, piece_option) in board_array.into_iter().enumerate() {
            if let Some(piece) = piece_option {
                board.add_piece(piece, ChessSquare::new(index as u8).unwrap());
            }
        }

        ChessGame {
            chessboard: board,
            side_to_move: if side_str == "w" { Color::White } else { Color::Black },
            castling_rights: CastlingRights::from_fen(castling_str),
            en_passant,
            halfmove_clock,
            fullmove_counter,
            game_history: Vec::new(),
            zobrist_hash: 0,
            rule_set: RuleSet::Legal,
        }
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank in (0..8).rev() {
            let mut empty = 0;

            for file in 0..8 {
                let sq = ChessSquare::from_coords(file, rank).unwrap();
                if let Some(ChessPiece { color, piece_type }) = self.chessboard.get_piece_at(sq) {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }

                    let c = match piece_type {
                        PieceType::Pawn => 'p',
                        PieceType::Knight => 'n',
                        PieceType::Bishop => 'b',
                        PieceType::Rook => 'r',
                        PieceType::Queen => 'q',
                        PieceType::King => 'k',
                    };

                    fen.push(if color == Color::White {
                        c.to_ascii_uppercase()
                    } else {
                        c
                    });
                } else {
                    empty += 1;
                }
            }

            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push(if self.side_to_move == Color::White { 'w' } else { 'b' });
        fen.push(' ');
        fen.push_str(&self.castling_rights.to_fen());
        fen.push(' ');
        fen.push_str(&self.en_passant.map_or("-".to_string(), |sq| sq.name()));
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.fullmove_counter.to_string());

        fen
    }

    pub fn uci_to_move(&self, input: &str) -> Result<ChessMove, &str> {
        let mut chars = input.chars();
        let from_str: String = chars.by_ref().take(2).collect();
        let to_str: String = chars.by_ref().take(2).collect();
        let promotion = chars.next();

        let from_sq = ChessSquare::from_name(&from_str).ok_or("Invalid from square")?;
        let to_sq = ChessSquare::from_name(&to_str).ok_or("Invalid to square")?;

        let promoted_type = if let Some(p) = promotion {
            match p {
                'q' => Some(PieceType::Queen),
                'r' => Some(PieceType::Rook),
                'b' => Some(PieceType::Bishop),
                'n' => Some(PieceType::Knight),
                _ => return Err("Invalid promotion"),
            }
        } else {
            None
        };

        Ok(ChessMove { from: from_sq, to: to_sq, promotion: promoted_type })
    }

    pub fn fen_to_ascii(fen: &str) {
        let mut board = String::new();
        let rows = fen.split_whitespace().next().unwrap_or("").split('/');

        for (rank_index, row) in rows.enumerate() {
            let mut line = format!("{} ", 8 - rank_index);
            for ch in row.chars() {
                if ch.is_ascii_digit() {
                    let empty = ch.to_digit(10).unwrap();
                    line.push_str(&". ".repeat(empty as usize));
                } else {
                    line.push(ch);
                    line.push(' ');
                }
            }
            board.push_str(line.trim_end());
            board.push('\n');
        }

        board.push_str("  a b c d e f g h\n");
        print!("{board}");
    }

    pub fn make_initial_hash(&mut self) {
        let mut hash = 0;
        let keys = ZobristKeys::get();

        for sq_idx in 0..64 {
            let sq = ChessSquare(sq_idx);
            if let Some(piece) = self.chessboard.get_piece_at(sq) {
                hash ^=
                    keys.pieces[piece.color as usize][piece.piece_type as usize][sq_idx as usize];
            }
        }

        hash ^= keys.castling[self.castling_rights.0 as usize];

        if let Some(sq) = self.en_passant {
            hash ^= keys.en_passant[sq.file() as usize];
        }

        if self.side_to_move == Color::Black {
            hash ^= keys.side_to_move;
        }

        self.zobrist_hash = hash;
    }

    pub fn validate_move(&self, mov: &ChessMove, legal: bool) -> Result<(), &str> {
        let from_sq = mov.from;
        let to_sq = mov.to;
        let opponent = self.side_to_move.opposite();

        let Some(from_piece) = self.chessboard.get_piece_at(from_sq) else {
            return Err("No piece selected");
        };

        if from_piece.color != self.side_to_move {
            return Err("Opponent Piece Selected");
        }

        if let Some(to_piece) = self.chessboard.get_piece_at(to_sq) {
            if to_piece.color == self.side_to_move {
                return Err("Ally piece targeted");
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
                            return Err("Cannot promote before reaching final rank");
                        }
                        if mov.to.rank() == 7
                            && (mov.promotion.is_none() || mov.promotion.is_some_and(|x| x == PieceType::Pawn))
                        {
                            return Err("Invalid promotion");
                        }
                    }
                    Color::Black => {
                        if mov.to.rank() != 0 && mov.promotion.is_some() {
                            return Err("Cannot promote before reaching final rank");
                        }
                        if mov.to.rank() == 0
                            && (mov.promotion.is_none() || mov.promotion.is_some_and(|x| x == PieceType::Pawn))
                        {
                            return Err("Invalid promotion");
                        }
                    }
                }

                if file_diff == 0 {
                    if rank_diff == direction {
                        if self.chessboard.all_pieces.is_set(to_sq) {
                            return Err("Pawn blocked");
                        }
                    } else if rank_diff == 2 * direction {
                        if from_sq.rank() != start_rank {
                            return Err("Invalid double push rank");
                        }
                        let mid_sq = ChessSquare((from_sq.0 as i8 + (8 * direction)) as u8);
                        if self.chessboard.all_pieces.is_set(to_sq) || self.chessboard.all_pieces.is_set(mid_sq) {
                            return Err("Pawn blocked");
                        }
                    } else {
                        return Err("Invalid pawn move");
                    }
                } else if file_diff == 1 {
                    if rank_diff != direction {
                        return Err("Invalid pawn capture direction");
                    }
                    let is_ep = self.en_passant.is_some_and(|sq| sq == to_sq);

                    if !is_ep {
                        let target_occupancy = match self.side_to_move {
                            Color::White => self.chessboard.black_occupancy,
                            Color::Black => self.chessboard.white_occupancy,
                        };

                        if !target_occupancy.is_set(to_sq) {
                            return Err("Pawn cannot capture empty square");
                        }
                    }
                } else {
                    return Err("Invalid pawn move");
                }
            }

            PieceType::Rook => {
                if !ChessBoard::ROOK_ATTACKS[from_sq.0 as usize].is_set(to_sq) {
                    return Err("Invalid Rook move");
                }
                let ray_board = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize]
                    .ok_or("Logic Error: Rook aligned but no BETWEEN mask")?;

                if !(ray_board & self.chessboard.all_pieces).is_empty() {
                    return Err("Rook Move Blocked");
                }
            }

            PieceType::Bishop => {
                if !ChessBoard::BISHOP_ATTACKS[from_sq.0 as usize].is_set(to_sq) {
                    return Err("Invalid Bishop move");
                }
                let ray_board = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize]
                    .ok_or("Logic Error: Bishop aligned but no BETWEEN mask")?;

                if !(ray_board & self.chessboard.all_pieces).is_empty() {
                    return Err("Bishop Move Blocked");
                }
            }

            PieceType::Knight => {
                if !ChessBoard::KNIGHT_ATTACKS[from_sq.0 as usize].is_set(to_sq) {
                    return Err("Invalid Knight move");
                }
            }

            PieceType::Queen => {
                let is_diag = ChessBoard::BISHOP_ATTACKS[from_sq.0 as usize].is_set(to_sq);
                let is_orth = ChessBoard::ROOK_ATTACKS[from_sq.0 as usize].is_set(to_sq);

                if !is_diag && !is_orth {
                    return Err("Invalid Queen move");
                }

                let ray_board = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize]
                    .ok_or("Logic Error: Queen aligned but no BETWEEN mask")?;

                if !(ray_board & self.chessboard.all_pieces).is_empty() {
                    return Err("Queen Move Blocked");
                }
            }

            PieceType::King => {
                if ChessBoard::KING_ATTACKS[from_sq.0 as usize].is_set(to_sq) {
                } else {
                    let between =
                        ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].ok_or("Invalid King Move")?;

                    if self.chessboard.all_pieces.is_set(to_sq) {
                        return Err("Cannot castle into occupied square");
                    }

                    if !(self.chessboard.all_pieces & between).is_empty() {
                        return Err("Castling path blocked");
                    }

                    if to_sq.file() == 2 {
                        let b_file_sq = if to_sq.rank() == 0 {
                            ChessSquare::B1
                        } else {
                            ChessSquare::B8
                        };
                        if self.chessboard.all_pieces.is_set(b_file_sq) {
                            return Err("Queenside castle blocked");
                        }
                    }

                    let (rights_needed, crossing_sq) = match to_sq {
                        ChessSquare::G1 => (CastlingRights::WHITE_KINGSIDE, ChessSquare::F1),
                        ChessSquare::C1 => (CastlingRights::WHITE_QUEENSIDE, ChessSquare::D1),
                        ChessSquare::G8 => (CastlingRights::BLACK_KINGSIDE, ChessSquare::F8),
                        ChessSquare::C8 => (CastlingRights::BLACK_QUEENSIDE, ChessSquare::D8),
                        _ => return Err("Invalid King Move"),
                    };

                    if !self.castling_rights.has(rights_needed) {
                        return Err("No castling rights");
                    }

                    if self.chessboard.is_square_attacked(from_sq, opponent) {
                        return Err("Cannot castle out of check");
                    }
                    if self.chessboard.is_square_attacked(crossing_sq, opponent) {
                        return Err("Cannot castle through check");
                    }
                    if self.chessboard.is_square_attacked(to_sq, opponent) {
                        return Err("Cannot castle into check");
                    }
                }
            }
        }

        if legal {
            let mut temp_board = self.chessboard.clone();
            temp_board.apply_move(&mov, self.side_to_move, self.en_passant);

            let king_bb = temp_board.get_piece_bitboard(self.side_to_move, PieceType::King);
            let king_sq = ChessSquare(king_bb.0.trailing_zeros() as u8);

            if temp_board.is_square_attacked(king_sq, opponent) {
                return Err("Move leaves King in check");
            }
        }

        Ok(())
    }

    pub fn make_move(&mut self, mov: &ChessMove) {
        let keys = ZobristKeys::get();
        let moving_piece = self.chessboard.get_piece_at(mov.from).expect("No piece at from sq");
        let captured_piece = self.chessboard.get_piece_at(mov.to);

        let is_en_passant =
            moving_piece.piece_type == PieceType::Pawn && self.en_passant.is_some_and(|sq| sq == mov.to);
        let is_castling =
            moving_piece.piece_type == PieceType::King && (mov.from.file() as i8 - mov.to.file() as i8).abs() == 2;

        self.game_history.push(GameStateEntry {
            move_made: mov.clone(),
            side_to_move: self.side_to_move,
            captured_piece: if is_en_passant {
                Some(ChessPiece::new(self.side_to_move.opposite(), PieceType::Pawn))
            } else {
                captured_piece
            },
            castling_rights: self.castling_rights,
            en_passant: self.en_passant,
            halfmove_clock: self.halfmove_clock,
            fullmove_counter: self.fullmove_counter,
            zobrist_hash: self.zobrist_hash,
        });

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

        if self.side_to_move == Color::Black {
            self.fullmove_counter += 1;
        }

        // Apply move needs ep square we just pushed
        let prev_ep = self.game_history.last().unwrap().en_passant;
        self.chessboard.apply_move(mov, self.side_to_move, prev_ep);

        self.side_to_move = self.side_to_move.opposite();

        // Hash global state
        self.zobrist_hash ^= keys.castling[self.castling_rights.0 as usize];
        if let Some(ep) = self.en_passant {
            self.zobrist_hash ^= keys.en_passant[ep.file() as usize];
        }
    }

    pub fn is_game_over(&self) -> Outcome {
        if self.halfmove_clock >= 100 {
            return Outcome::Finished(None);
        }

        let repetition_count = self.game_history.iter().filter(|entry| entry.zobrist_hash == self.zobrist_hash).count();

        if repetition_count >= 2 {
            return Outcome::Finished(None);
        }

        if self.is_insufficient_material() {
            return Outcome::Finished(None);
        }

        let white_king = self.chessboard.get_piece_bitboard(Color::White, PieceType::King);
        let black_king = self.chessboard.get_piece_bitboard(Color::Black, PieceType::King);

        if white_king.is_empty() {
            return Outcome::Finished(Some(Color::Black));
        }
        if black_king.is_empty() {
            return Outcome::Finished(Some(Color::White));
        }

        Outcome::Unfinished
    }

    fn is_checkmate(&self) -> bool {
        let mut king_bb = self.chessboard.get_piece_bitboard(self.side_to_move.opposite(), PieceType::King);
        let king_sq = king_bb.pop_lsb().unwrap();
        if !self.chessboard.is_square_attacked(king_sq, self.side_to_move.opposite()) {
            return false;
        }
        let mut squares = ChessBoard::KING_ATTACKS[king_sq.0 as usize];
        while let Some(square) = squares.pop_lsb() {
            let occupancy = match self.side_to_move {
                Color::White => self.chessboard.white_occupancy,
                Color::Black => self.chessboard.black_occupancy,
            };
            if !self.chessboard.is_square_attacked(square, self.side_to_move) || (king_bb | occupancy).is_empty() {
                return false;
            }
        }
        return true;
    }

    fn is_insufficient_material(&self) -> bool {
        let all_pieces = self.chessboard.all_pieces;
        let count = all_pieces.count();

        if count == 2 {
            return true;
        }

        if count == 3 {
            let white_minors = self.chessboard.get_piece_bitboard(Color::White, PieceType::Knight)
                | self.chessboard.get_piece_bitboard(Color::White, PieceType::Bishop);
            let black_minors = self.chessboard.get_piece_bitboard(Color::Black, PieceType::Knight)
                | self.chessboard.get_piece_bitboard(Color::Black, PieceType::Bishop);

            if !white_minors.is_empty() || !black_minors.is_empty() {
                return true;
            }
        }
        false
    }
}

use super::{
    Bitboard, CastlingRights, ChessBoard, ChessMove, ChessPiece, ChessSquare, Color, PieceType,
};
use std::collections::{HashMap, btree_map::Keys};

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
}

#[derive(Debug, Clone)]
pub struct ChessGame {
    pub board: ChessBoard,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<ChessSquare>,
    pub halfmove_clock: u32,
    pub fullmove_counter: u32,
    pub position_history: HashMap<u64, u32>,
    pub game_history: Vec<GameStateEntry>,
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
        let halfmove_clock: u32 = parts
            .next()
            .expect("FEN missing halfmove clock")
            .parse()
            .expect("Invalid halfmove clock");
        let fullmove_counter: u32 = parts
            .next()
            .expect("FEN missing fullmove counter")
            .parse()
            .expect("Invalid fullmove counter");

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
                    let color = if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };

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
            board,
            side_to_move: if side_str == "w" {
                Color::White
            } else {
                Color::Black
            },
            castling_rights: CastlingRights::from_fen(castling_str),
            en_passant,
            halfmove_clock,
            fullmove_counter,
            position_history: HashMap::new(),
            game_history: Vec::new(),
        }
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank in (0..8).rev() {
            let mut empty = 0;

            for file in 0..8 {
                let sq = ChessSquare::from_coords(file, rank).unwrap();
                if let Some(ChessPiece { color, piece_type }) = self.board.get_piece_at(sq) {
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
        fen.push(if self.side_to_move == Color::White {
            'w'
        } else {
            'b'
        });
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

        Ok(ChessMove {
            from: from_sq,
            to: to_sq,
            promotion: promoted_type,
        })
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

    // fn compute_position_hash(&self) -> u64 {

    // }

    pub fn validate_move(&self, mov: &ChessMove, legal: bool) -> Result<(), &str> {
        let from_sq = mov.from;
        let to_sq = mov.to;
        let opponent = self.side_to_move.opposite();

        let Some(from_piece) = self.board.get_piece_at(from_sq) else {
            return Err("No piece selected");
        };

        if from_piece.color != self.side_to_move {
            return Err("Opponent Piece Selected");
        }

        if let Some(to_piece) = self.board.get_piece_at(to_sq) {
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

                if file_diff == 0 {
                    if rank_diff == direction {
                        if self.board.all_pieces.is_set(to_sq) {
                            return Err("Pawn blocked");
                        }
                    } else if rank_diff == 2 * direction {
                        if from_sq.rank() != start_rank {
                            return Err("Invalid double push rank");
                        }
                        let mid_sq = ChessSquare((from_sq.0 as i8 + (8 * direction)) as u8);
                        if self.board.all_pieces.is_set(to_sq)
                            || self.board.all_pieces.is_set(mid_sq)
                        {
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
                            Color::White => self.board.black_occupancy,
                            Color::Black => self.board.white_occupancy,
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

                if !(ray_board & self.board.all_pieces).is_empty() {
                    return Err("Rook Move Blocked");
                }
            }

            PieceType::Bishop => {
                if !ChessBoard::BISHOP_ATTACKS[from_sq.0 as usize].is_set(to_sq) {
                    return Err("Invalid Bishop move");
                }
                let ray_board = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize]
                    .ok_or("Logic Error: Bishop aligned but no BETWEEN mask")?;

                if !(ray_board & self.board.all_pieces).is_empty() {
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

                if !(ray_board & self.board.all_pieces).is_empty() {
                    return Err("Queen Move Blocked");
                }
            }

            PieceType::King => {
                if ChessBoard::KING_ATTACKS[from_sq.0 as usize].is_set(to_sq) {
                } else {
                    let between = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize]
                        .ok_or("Invalid King Move")?;

                    if !(self.board.all_pieces & between).is_empty() {
                        return Err("Castling path blocked");
                    }

                    if to_sq.file() == 2 {
                        let b_file_sq = if to_sq.rank() == 0 {
                            ChessSquare::B1
                        } else {
                            ChessSquare::B8
                        };
                        if self.board.all_pieces.is_set(b_file_sq) {
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

                    if self.is_square_attacked(from_sq, opponent) {
                        return Err("Cannot castle out of check");
                    }
                    if self.is_square_attacked(crossing_sq, opponent) {
                        return Err("Cannot castle through check");
                    }
                    if self.is_square_attacked(to_sq, opponent) {
                        return Err("Cannot castle into check");
                    }
                }
            }
        }

        if legal {
            let mut temp_board = self.clone();
            temp_board.make_move(mov);

            let our_color = self.side_to_move;

            let king_bitboard = self.board.pieces[our_color as usize][PieceType::King as usize];
            let king_sq = ChessSquare(king_bitboard.0.trailing_zeros() as u8);

            if self.is_square_attacked(king_sq, opponent) {
                return Err("Move leaves King in check");
            }
        }

        Ok(())
    }

    pub fn make_move(&mut self, mv: &ChessMove) {
        let moving_piece = self.board.get_piece_at(mv.from).expect("No piece selected");
        let mut captured_piece = self.board.get_piece_at(mv.to);

        let is_en_passant = moving_piece.piece_type == PieceType::Pawn
            && self.en_passant.is_some_and(|sq| sq == mv.to);

        if is_en_passant {
            captured_piece = Some(ChessPiece {
                color: self.side_to_move.opposite(),
                piece_type: PieceType::Pawn,
            });
        }

        self.game_history.push(GameStateEntry {
            move_made: mv.clone(),
            side_to_move: self.side_to_move,
            captured_piece,
            castling_rights: self.castling_rights,
            en_passant: self.en_passant,
            halfmove_clock: self.halfmove_clock,
            fullmove_counter: self.fullmove_counter,
            zobrist_hash: 0, // TODO
        });

        self.board.move_piece(mv.from, mv.to, moving_piece);

        if let Some(promo_piece_type) = mv.promotion {
            self.board.remove_piece(moving_piece, mv.to);
            let new_piece = ChessPiece {
                color: self.side_to_move,
                piece_type: promo_piece_type,
            };
            self.board.add_piece(new_piece, mv.to);
        }

        if is_en_passant {
            let captured_square = if self.side_to_move == Color::White {
                ChessSquare(mv.to.0 - 8)
            } else {
                ChessSquare(mv.to.0 + 8)
            };
            let captured_pawn = ChessPiece {
                color: self.side_to_move.opposite(),
                piece_type: PieceType::Pawn,
            };
            self.board.remove_piece(captured_pawn, captured_square);
        }

        if moving_piece.piece_type == PieceType::King
            && (mv.from.file() as i8 - mv.to.file() as i8).abs() == 2
        {
            let (rook_from, rook_to) = match (self.side_to_move, mv.to.file()) {
                (Color::White, f) if f > mv.from.file() => (ChessSquare::H1, ChessSquare::F1),
                (Color::White, _) => (ChessSquare::A1, ChessSquare::D1),
                (Color::Black, f) if f > mv.from.file() => (ChessSquare::H8, ChessSquare::F8),
                (Color::Black, _) => (ChessSquare::A8, ChessSquare::D8),
            };
            let rook = ChessPiece {
                color: self.side_to_move,
                piece_type: PieceType::Rook,
            };
            self.board.move_piece(rook_from, rook_to, rook);
        }

        let mut rights_to_remove = CastlingRights::empty();

        if moving_piece.piece_type == PieceType::King {
            match self.side_to_move {
                Color::White => {
                    rights_to_remove |=
                        CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE;
                }
                Color::Black => {
                    rights_to_remove |=
                        CastlingRights::BLACK_KINGSIDE | CastlingRights::BLACK_QUEENSIDE;
                }
            }
        }

        match mv.from {
            ChessSquare::H1 => rights_to_remove |= CastlingRights::WHITE_KINGSIDE,
            ChessSquare::A1 => rights_to_remove |= CastlingRights::WHITE_QUEENSIDE,
            ChessSquare::H8 => rights_to_remove |= CastlingRights::BLACK_KINGSIDE,
            ChessSquare::A8 => rights_to_remove |= CastlingRights::BLACK_QUEENSIDE,
            _ => {}
        }

        match mv.to {
            ChessSquare::H1 => rights_to_remove |= CastlingRights::WHITE_KINGSIDE,
            ChessSquare::A1 => rights_to_remove |= CastlingRights::WHITE_QUEENSIDE,
            ChessSquare::H8 => rights_to_remove |= CastlingRights::BLACK_KINGSIDE,
            ChessSquare::A8 => rights_to_remove |= CastlingRights::BLACK_QUEENSIDE,
            _ => {}
        }

        self.castling_rights.remove(rights_to_remove);

        self.en_passant = None;
        if moving_piece.piece_type == PieceType::Pawn
            && (mv.from.rank() as i8 - mv.to.rank() as i8).abs() == 2
        {
            let skipped_square = ChessSquare((mv.from.0 + mv.to.0) / 2);
            self.en_passant = Some(skipped_square);
        }

        if moving_piece.piece_type == PieceType::Pawn || captured_piece.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        if self.side_to_move == Color::Black {
            self.fullmove_counter += 1;
        }

        self.side_to_move = self.side_to_move.opposite();
    }

    pub fn unmake_move(&mut self) {
        let entry = self.game_history.pop().expect("No history to unmake");
        let mov = entry.move_made;

        self.side_to_move = entry.side_to_move;
        self.castling_rights = entry.castling_rights;
        self.en_passant = entry.en_passant;
        self.halfmove_clock = entry.halfmove_clock;
        self.fullmove_counter = entry.fullmove_counter;

        let current_piece = self
            .board
            .get_piece_at(mov.to)
            .expect("Board desync: Piece missing on unmake");

        if mov.promotion.is_some() {
            self.board.remove_piece(current_piece, mov.to);
            let pawn = ChessPiece {
                color: self.side_to_move,
                piece_type: PieceType::Pawn,
            };
            self.board.add_piece(pawn, mov.from);
        } else {
            self.board.move_piece(mov.to, mov.from, current_piece);
        }

        if let Some(cap_piece) = entry.captured_piece {
            let mut cap_sq = mov.to;

            if current_piece.piece_type == PieceType::Pawn && entry.en_passant == Some(mov.to) {
                cap_sq = ChessSquare::from_coords(mov.to.file(), mov.from.rank()).unwrap();
            }

            self.board.add_piece(cap_piece, cap_sq);
        }

        if current_piece.piece_type == PieceType::King
            && (mov.from.file() as i8 - mov.to.file() as i8).abs() == 2
        {
            let (rook_now, rook_orig) = match mov.to {
                ChessSquare::G1 => (ChessSquare::F1, ChessSquare::H1),
                ChessSquare::C1 => (ChessSquare::D1, ChessSquare::A1),
                ChessSquare::G8 => (ChessSquare::F8, ChessSquare::H8),
                ChessSquare::C8 => (ChessSquare::D8, ChessSquare::A8),
                _ => panic!("Invalid castle unmake state"),
            };

            let rook = self
                .board
                .get_piece_at(rook_now)
                .expect("Rook missing in un-castle");
            self.board.move_piece(rook_now, rook_orig, rook);
        }
    }
}

use burn::{Tensor, prelude::Backend};

use super::{Bitboard, CastlingRights, ChessBoard, ChessMove, ChessPiece, ChessSquare, Color, PieceType, ZobristKeys};

pub enum Outcome {
    Unfinished,
    Finished(Option<Color>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleSet {
    Legal,
    PseudoLegal,
}

#[derive(Debug, Clone)]
pub struct GameStateEntry {
    pub chessboard: ChessBoard,
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
                    file += c.to_digit(10).expect("from_fen") as u8;
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
                board.add_piece(piece, ChessSquare::new(index as u8).expect("from_fen"));
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

    pub fn calculate_hash(&mut self) -> u64 {
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

    // unused
    // pub fn validate_move(&self, mov: &ChessMove) -> MoveValidity {
    //     let from_sq = mov.from;
    //     let to_sq = mov.to;
    //     let opponent = self.side_to_move.opposite();
    //
    //     let Some(from_piece) = self.chessboard.get_piece_at(from_sq) else {
    //         return MoveValidity::Impossible;
    //     };
    //
    //     if from_piece.color != self.side_to_move {
    //         return MoveValidity::Impossible;
    //     }
    //
    //     if let Some(to_piece) = self.chessboard.get_piece_at(to_sq) {
    //         if to_piece.color == self.side_to_move {
    //             return MoveValidity::Impossible;
    //         }
    //     }
    //
    //     match from_piece.piece_type {
    //         PieceType::Pawn => {
    //             let (direction, start_rank) = match self.side_to_move {
    //                 Color::White => (1, 1),
    //                 Color::Black => (-1, 6),
    //             };
    //
    //             let rank_diff = to_sq.rank() as i8 - from_sq.rank() as i8;
    //             let file_diff = (to_sq.file() as i8 - from_sq.file() as i8).abs();
    //
    //             match self.side_to_move {
    //                 Color::White => {
    //                     if mov.to.rank() != 7 && mov.promotion.is_some() {
    //                         return MoveValidity::Impossible;
    //                     }
    //                     if mov.to.rank() == 7
    //                         && (mov.promotion.is_none() || mov.promotion.is_some_and(|x| x == PieceType::Pawn))
    //                     {
    //                         return MoveValidity::Impossible;
    //                     }
    //                 }
    //                 Color::Black => {
    //                     if mov.to.rank() != 0 && mov.promotion.is_some() {
    //                         return MoveValidity::Impossible;
    //                     }
    //                     if mov.to.rank() == 0
    //                         && (mov.promotion.is_none() || mov.promotion.is_some_and(|x| x == PieceType::Pawn))
    //                     {
    //                         return MoveValidity::Impossible;
    //                     }
    //                 }
    //             }
    //
    //             if file_diff == 0 {
    //                 if rank_diff == direction {
    //                     if self.chessboard.all_pieces.is_set(to_sq) {
    //                         return MoveValidity::Impossible;
    //                     }
    //                 } else if rank_diff == 2 * direction {
    //                     if from_sq.rank() != start_rank {
    //                         return MoveValidity::Impossible;
    //                     }
    //                     let mid_sq = ChessSquare((from_sq.0 as i8 + (8 * direction)) as u8);
    //                     if self.chessboard.all_pieces.is_set(to_sq) || self.chessboard.all_pieces.is_set(mid_sq) {
    //                         return MoveValidity::Impossible;
    //                     }
    //                 } else {
    //                     return MoveValidity::Impossible;
    //                 }
    //             } else if file_diff == 1 {
    //                 if rank_diff != direction {
    //                     return MoveValidity::Impossible;
    //                 }
    //                 let is_ep = self.en_passant.is_some_and(|sq| sq == to_sq);
    //
    //                 if !is_ep {
    //                     let target_occupancy = match self.side_to_move {
    //                         Color::White => self.chessboard.black_occupancy,
    //                         Color::Black => self.chessboard.white_occupancy,
    //                     };
    //
    //                     if !target_occupancy.is_set(to_sq) {
    //                         return MoveValidity::Impossible;
    //                     }
    //                 }
    //             } else {
    //                 return MoveValidity::Impossible;
    //             }
    //         }
    //
    //         PieceType::Rook => {
    //             let rook_attacks = ChessBoard::ROOK_ATTACKS[from_sq.0 as usize]
    //                 .iter()
    //                 .copied()
    //                 .reduce(|acc, bb| acc | bb)
    //                 .unwrap_or(Bitboard::EMPTY);
    //             if !rook_attacks.is_set(to_sq) {
    //                 return MoveValidity::Impossible;
    //             }
    //             let ray_board = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].unwrap();
    //
    //             if !(ray_board & self.chessboard.all_pieces).is_empty() {
    //                 return MoveValidity::Impossible;
    //             }
    //         }
    //
    //         PieceType::Bishop => {
    //             let bishop_attacks = ChessBoard::BISHOP_ATTACKS[from_sq.0 as usize]
    //                 .iter()
    //                 .copied()
    //                 .reduce(|acc, bb| acc | bb)
    //                 .unwrap_or(Bitboard::EMPTY);
    //             if !bishop_attacks.is_set(to_sq) {
    //                 return MoveValidity::Impossible;
    //             }
    //             let ray_board = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].unwrap();
    //
    //             if !(ray_board & self.chessboard.all_pieces).is_empty() {
    //                 return MoveValidity::Impossible;
    //             }
    //         }
    //
    //         PieceType::Knight => {
    //             if !ChessBoard::KNIGHT_ATTACKS[from_sq.0 as usize].is_set(to_sq) {
    //                 return MoveValidity::Impossible;
    //             }
    //         }
    //
    //         PieceType::Queen => {
    //             let bishop_attacks = ChessBoard::BISHOP_ATTACKS[from_sq.0 as usize]
    //                 .iter()
    //                 .copied()
    //                 .reduce(|acc, bb| acc | bb)
    //                 .unwrap_or(Bitboard::EMPTY);
    //             let rook_attacks = ChessBoard::ROOK_ATTACKS[from_sq.0 as usize]
    //                 .iter()
    //                 .copied()
    //                 .reduce(|acc, bb| acc | bb)
    //                 .unwrap_or(Bitboard::EMPTY);
    //
    //             let is_diag = bishop_attacks.is_set(to_sq);
    //             let is_orth = rook_attacks.is_set(to_sq);
    //
    //             if !is_diag && !is_orth {
    //                 return MoveValidity::Impossible;
    //             }
    //
    //             let ray_board = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].unwrap();
    //
    //             if !(ray_board & self.chessboard.all_pieces).is_empty() {
    //                 return MoveValidity::Impossible;
    //             }
    //         }
    //
    //         PieceType::King => {
    //             let between = ChessBoard::BETWEEN[from_sq.0 as usize][to_sq.0 as usize].unwrap();
    //
    //             if self.chessboard.all_pieces.is_set(to_sq) {
    //                 return MoveValidity::Impossible;
    //             }
    //
    //             if !(self.chessboard.all_pieces & between).is_empty() {
    //                 return MoveValidity::Impossible;
    //             }
    //
    //             if to_sq.file() == 2 {
    //                 let b_file_sq = if to_sq.rank() == 0 {
    //                     ChessSquare::B1
    //                 } else {
    //                     ChessSquare::B8
    //                 };
    //                 if self.chessboard.all_pieces.is_set(b_file_sq) {
    //                     return MoveValidity::Impossible;
    //                 }
    //             }
    //
    //             let (rights_needed, crossing_sq) = match to_sq {
    //                 ChessSquare::G1 => (CastlingRights::WHITE_KINGSIDE, ChessSquare::F1),
    //                 ChessSquare::C1 => (CastlingRights::WHITE_QUEENSIDE, ChessSquare::D1),
    //                 ChessSquare::G8 => (CastlingRights::BLACK_KINGSIDE, ChessSquare::F8),
    //                 ChessSquare::C8 => (CastlingRights::BLACK_QUEENSIDE, ChessSquare::D8),
    //                 _ => return MoveValidity::Impossible,
    //             };
    //
    //             if !self.castling_rights.has(rights_needed) {
    //                 return MoveValidity::Impossible;
    //             }
    //
    //             if self.chessboard.is_square_attacked(from_sq, opponent) {
    //                 return MoveValidity::Impossible;
    //             }
    //             if self.chessboard.is_square_attacked(crossing_sq, opponent) {
    //                 return MoveValidity::Impossible;
    //             }
    //             if self.chessboard.is_square_attacked(to_sq, opponent) {
    //                 if self.rule_set == RuleSet::Legal {
    //                     return MoveValidity::Impossible;
    //                 } else {
    //                     return MoveValidity::PseudoLegal;
    //                 }
    //             }
    //
    //             if !ChessBoard::KING_ATTACKS[from_sq.0 as usize].is_set(to_sq) {
    //                 return MoveValidity::Impossible;
    //             }
    //         }
    //     }
    //     // All geometry checks done, so must be pseudo legal or legal
    //     if self.rule_set == RuleSet::Legal && self.is_legal(mov) {
    //         return MoveValidity::Legal;
    //     } else {
    //         return MoveValidity::PseudoLegal;
    //     }
    // }

    pub fn is_legal(&self, mov: &ChessMove) -> bool {
        // I DONT CARE! its 120 bytes its FINE.
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

    pub fn is_valid(&self, mov: &ChessMove) -> bool {
        let pseudolegal_moves = self.generate_pseudolegal();
        if pseudolegal_moves.contains(mov) {
            if self.rule_set == RuleSet::PseudoLegal {
                return true;
            } else if self.is_legal(mov) {
                return true;
            }
        }
        false
    }

    pub fn get_possible_from_squares(&self) -> Vec<ChessSquare> {
        self.generate_pseudolegal().clone().into_iter().map(|mov| mov.from).collect()
    }

    pub fn get_possible_to_squares(&self, from_square: &ChessSquare) -> Vec<ChessSquare> {
        self.generate_pseudolegal()
            .clone()
            .into_iter()
            .filter(|mov| mov.from == *from_square)
            .map(|mov| mov.to)
            .collect()
    }

    // should make pseudolegal/legal moves indiscriminantly. should never be passed impossible
    // moves.
    pub fn make_move(&mut self, mov: &ChessMove) {
        let keys = ZobristKeys::get();
        let moving_piece = self.chessboard.get_piece_at(mov.from).expect("No piece at from sq");
        let captured_piece = self.chessboard.get_piece_at(mov.to);

        let is_en_passant =
            moving_piece.piece_type == PieceType::Pawn && self.en_passant.is_some_and(|sq| sq == mov.to);
        let is_castling =
            moving_piece.piece_type == PieceType::King && (mov.from.file() as i8 - mov.to.file() as i8).abs() == 2;

        self.game_history.push(GameStateEntry {
            chessboard: self.chessboard.clone(),
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

        debug_assert!(self.zobrist_hash == self.calculate_hash())
    }

    // not needed, cloning bitboards instead
    pub fn unmake_move(&mut self) {
        let entry = self.game_history.pop().expect("No history to unmake");
        let mov = entry.move_made;
        self.side_to_move = entry.side_to_move;
        self.castling_rights = entry.castling_rights;
        self.en_passant = entry.en_passant;
        self.halfmove_clock = entry.halfmove_clock;
        self.fullmove_counter = entry.fullmove_counter;

        let current_piece = self.chessboard.get_piece_at(mov.to).expect("chessboard desync: Piece missing on unmake");

        if mov.promotion.is_some() {
            self.chessboard.remove_piece(current_piece, mov.to);
            let pawn = ChessPiece { color: self.side_to_move, piece_type: PieceType::Pawn };
            self.chessboard.add_piece(pawn, mov.from);
        } else {
            self.chessboard.move_piece(mov.to, mov.from, current_piece);
        }

        if let Some(cap_piece) = entry.captured_piece {
            let mut cap_sq = mov.to;
            if current_piece.piece_type == PieceType::Pawn && entry.en_passant == Some(mov.to) {
                cap_sq = ChessSquare::from_coords(mov.to.file(), mov.from.rank()).unwrap();
            }
            self.chessboard.add_piece(cap_piece, cap_sq);
        }

        if current_piece.piece_type == PieceType::King && (mov.from.file() as i8 - mov.to.file() as i8).abs() == 2 {
            let (rook_now, rook_orig) = match mov.to {
                ChessSquare::G1 => (ChessSquare::F1, ChessSquare::H1),
                ChessSquare::C1 => (ChessSquare::D1, ChessSquare::A1),
                ChessSquare::G8 => (ChessSquare::F8, ChessSquare::H8),
                ChessSquare::C8 => (ChessSquare::D8, ChessSquare::A8),
                _ => panic!("Invalid castle unmake state"),
            };

            let rook = self.chessboard.get_piece_at(rook_now).expect("Rook missing in un-castle");

            self.chessboard.move_piece(rook_now, rook_orig, rook);
        }
    }

    pub fn check_game_state(&self) -> Outcome {
        // PseudoLegal and Legal Checks
        if self.halfmove_clock >= 100 {
            return Outcome::Finished(None);
        }

        let repetition_count = self.game_history.iter().filter(|entry| entry.zobrist_hash == self.zobrist_hash).count();

        if repetition_count >= 2 {
            return Outcome::Finished(None);
        }

        // King capture
        let white_king = self.chessboard.get_piece_bitboard(Color::White, PieceType::King);
        let black_king = self.chessboard.get_piece_bitboard(Color::Black, PieceType::King);

        if white_king.is_empty() {
            return Outcome::Finished(Some(Color::Black));
        }
        if black_king.is_empty() {
            return Outcome::Finished(Some(Color::White));
        }

        if self.rule_set == RuleSet::PseudoLegal {
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
}

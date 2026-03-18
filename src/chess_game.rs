use super::{
    CastlingRights, ChessBoard, ChessMove, ChessPiece, ChessPosition, ChessSquare, Color, PieceType, ZobristKeys,
};

#[derive(Debug, Clone, PartialEq, Eq)]
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
pub struct ChessGame {
    pub position: ChessPosition,
    pub halfmove_clock: u32,
    pub fullmove_counter: u32,
    pub game_history: Vec<ChessPosition>,
    pub rule_set: RuleSet,
    pub outcome: Outcome,
}

impl Default for ChessGame {
    fn default() -> Self {
        let mut game = ChessGame::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let hash = game.position.calculate_hash();
        game.position.zobrist_hash = hash;
        game
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

        let mut chessboard = ChessBoard::empty();
        for (index, piece_option) in board_array.into_iter().enumerate() {
            if let Some(piece) = piece_option {
                chessboard.add_piece(piece, ChessSquare::new(index as u8).expect("from_fen"));
            }
        }

        let position = ChessPosition {
            chessboard,
            side_to_move: if side_str == "w" { Color::White } else { Color::Black },
            castling_rights: CastlingRights::from_fen(castling_str),
            en_passant,
            halfmove_clock,
            zobrist_hash: 0,
        };

        ChessGame {
            position,
            halfmove_clock,
            fullmove_counter,
            game_history: Vec::new(),
            rule_set: RuleSet::Legal,
            outcome: Outcome::Unfinished,
        }
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank in (0..8).rev() {
            let mut empty = 0;

            for file in 0..8 {
                let sq = ChessSquare::from_coords(file, rank).unwrap();
                if let Some(ChessPiece { color, piece_type }) = self.position.chessboard.get_piece_at(sq) {
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
        fen.push(if self.position.side_to_move == Color::White {
            'w'
        } else {
            'b'
        });
        fen.push(' ');
        fen.push_str(&self.position.castling_rights.to_fen());
        fen.push(' ');
        fen.push_str(&self.position.en_passant.map_or("-".to_string(), |sq| sq.name()));
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

    // should make pseudolegal/legal moves indiscriminantly. should never be passed impossible
    // moves.
    pub fn make_move(mut self, mov: &ChessMove) {
        if self.position.side_to_move == Color::Black {
            self.fullmove_counter += 1;
        }
        self.position = self.position.make_move(mov);

        self.game_history.push(self.position.clone());
    }

    pub fn unmake_move(&mut self) {
        // this defeats the purpose of make unmake. who cares
        let entry = self.game_history.pop().expect("No history to unmake");
        self.position = entry;
        self.halfmove_clock -= 1;
        self.position.zobrist_hash = self.position.calculate_hash();

        // let current_piece = self.chessboard.get_piece_at(mov.to).expect("chessboard desync: Piece missing on unmake");
        //
        // if mov.promotion.is_some() {
        //     self.chessboard.remove_piece(current_piece, mov.to);
        //     let pawn = ChessPiece { color: self.side_to_move, piece_type: PieceType::Pawn };
        //     self.chessboard.add_piece(pawn, mov.from);
        // } else {
        //     self.chessboard.move_piece(mov.to, mov.from, current_piece);
        // }
        //
        // if let Some(cap_piece) = entry.captured_piece {
        //     let mut cap_sq = mov.to;
        //     if current_piece.piece_type == PieceType::Pawn && entry.en_passant == Some(mov.to) {
        //         cap_sq = ChessSquare::from_coords(mov.to.file(), mov.from.rank()).unwrap();
        //     }
        //     self.chessboard.add_piece(cap_piece, cap_sq);
        // }
        //
        // if current_piece.piece_type == PieceType::King && (mov.from.file() as i8 - mov.to.file() as i8).abs() == 2 {
        //     let (rook_now, rook_orig) = match mov.to {
        //         ChessSquare::G1 => (ChessSquare::F1, ChessSquare::H1),
        //         ChessSquare::C1 => (ChessSquare::D1, ChessSquare::A1),
        //         ChessSquare::G8 => (ChessSquare::F8, ChessSquare::H8),
        //         ChessSquare::C8 => (ChessSquare::D8, ChessSquare::A8),
        //         _ => panic!("Invalid castle unmake state"),
        //     };
        //
        //     let rook = self.chessboard.get_piece_at(rook_now).expect("Rook missing in un-castle");
        //
        // self.chessboard.move_piece(rook_now, rook_orig, rook);
        //}
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
        let white_king = self.position.chessboard.get_piece_bitboard(Color::White, PieceType::King);
        let black_king = self.position.chessboard.get_piece_bitboard(Color::Black, PieceType::King);

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
        let mut king_bb = self.position.chessboard.get_piece_bitboard(self.position.side_to_move, PieceType::King);
        // Safe, king must exist otherwise wouldve returned earlier
        let king_sq = king_bb.pop_lsb().unwrap();
        let mut legal_moves = self.position.generate_pseudolegal();
        legal_moves.retain(|x| self.position.is_legal(x));

        if legal_moves.is_empty() {
            if self.position.chessboard.is_square_attacked(king_sq, self.position.side_to_move.opposite()) {
                return Outcome::Finished(Some(self.position.side_to_move.opposite()));
            } else {
                return Outcome::Finished(None);
            }
        }

        // insufficient material
        let all_pieces = self.position.chessboard.all_pieces;
        let count = all_pieces.count();

        if count == 2 {
            return Outcome::Finished(None);
        }
        let mut white_bishops = self.position.chessboard.get_piece_bitboard(Color::White, PieceType::Bishop);
        let white_knights = self.position.chessboard.get_piece_bitboard(Color::White, PieceType::Knight);
        let mut black_bishops = self.position.chessboard.get_piece_bitboard(Color::Black, PieceType::Bishop);
        let black_knights = self.position.chessboard.get_piece_bitboard(Color::Black, PieceType::Knight);

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

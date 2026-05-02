use arrayvec::ArrayVec;

use super::{CastlingRights, ChessBoard, ChessMove, ChessPiece, ChessPosition, ChessSquare, Color, PieceType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Unfinished,
    Finished(Option<Color>),
}

impl Outcome {
    pub fn to_f32(&self) -> Option<[f32; 3]> {
        match self {
            Outcome::Finished(Some(Color::White)) => Some([1.0, 0.0, 0.0]),
            Outcome::Finished(None) => Some([0.0, 1.0, 0.0]),
            Outcome::Finished(Some(Color::Black)) => Some([0.0, 0.0, 1.0]),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChessGame {
    // this holds global data for the mcts arena
    pub position: ChessPosition,
    pub fullmove_counter: u32,
    pub game_history: Vec<ChessPosition>,
    pub outcome: Outcome,
}

impl Default for ChessGame {
    fn default() -> Self {
        let mut game = ChessGame::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let hash = game.position.calculate_hash();
        game.position.zobrist_hash = hash;
        game.game_history.push(game.position.clone());
        game
    }
}

impl ChessGame {
    pub fn from_fen(fen: &str) -> Result<Self, &str> {
        let mut parts = fen.split(' ');
        let board_str = parts.next().ok_or("FEN missing board")?;
        let side_str = parts.next().ok_or("FEN missing side to move")?;
        let castling_str = parts.next().ok_or("FEN missing castling rights")?;
        let ep_str = parts.next().ok_or("FEN missing en passant square")?;
        let halfmove_clock: u32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        let fullmove_counter: u32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(1);

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
                    file += c.to_digit(10).ok_or("from_fen")? as u8;
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
                        _ => return Err("Invalid piece type"),
                    };

                    let index = (rank as usize) * 8 + (file as usize);

                    board_array[index] = Some(ChessPiece::new(color, piece_type));
                    file += 1;
                }
            }
        }

        let en_passant = match ep_str {
            "-" => None,
            s => Some(ChessSquare::from_name(s).ok_or("Invalid en passant square")?),
        };

        let mut chessboard = ChessBoard::empty();
        for (index, piece_option) in board_array.into_iter().enumerate() {
            if let Some(piece) = piece_option {
                chessboard.add_piece(piece, ChessSquare::new(index as u8).ok_or("from_fen")?);
            }
        }

        let mut position = ChessPosition {
            chessboard,
            side_to_move: if side_str == "w" { Color::White } else { Color::Black },
            castling_rights: CastlingRights::from_fen(castling_str),
            en_passant,
            halfmove_clock,
            zobrist_hash: 0,
            pseudolegal_moves: ArrayVec::<ChessMove, 128>::new(),
        };

        position.generate_pseudolegal();

        Ok(ChessGame { position, fullmove_counter, game_history: Vec::new(), outcome: Outcome::Unfinished })
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

                    fen.push(if color == Color::White { c.to_ascii_uppercase() } else { c });
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
        fen.push(if self.position.side_to_move == Color::White { 'w' } else { 'b' });
        fen.push(' ');
        fen.push_str(&self.position.castling_rights.to_fen());
        fen.push(' ');
        fen.push_str(&self.position.en_passant.map_or("-".to_string(), |sq| sq.name()));
        fen.push(' ');
        fen.push_str(&self.position.halfmove_clock.to_string());
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

    // should make pseudolegal/legal moves indiscriminantly. should never be passed impossible moves.
    pub fn make_move(&mut self, mov: &ChessMove) {
        if self.position.side_to_move == Color::Black {
            self.fullmove_counter += 1;
        }
        self.position.make_move(mov);

        self.game_history.push(self.position.clone());
        self.position.generate_pseudolegal();
    }

    pub fn unmake_move(&mut self) {
        // this defeats the purpose of make unmake. unused.
        let entry = self.game_history.pop().expect("No history to unmake");
        self.position = entry;
        self.position.halfmove_clock -= 1;
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

    pub fn check_game_state(&self, legal: bool) -> Outcome {
        let repetition_count = self.game_history.iter().filter(|entry| entry.zobrist_hash == self.position.zobrist_hash).count();

        if repetition_count >= 3 {
            return Outcome::Finished(None);
        }

        self.position.check_game_state(legal)
    }
}

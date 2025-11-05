use super::{
    Bitboard, CastlingRights, ChessBoard, ChessMove, ChessPiece, ChessSquare, Color, PieceType,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ChessGame {
    pub board: ChessBoard,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<ChessSquare>,
    pub halfmove_clock: u32,
    pub fullmove_counter: u32,
    pub position_history: HashMap<u64, u32>,
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

    pub fn validate_move(&self, mov: &mut ChessMove) -> Result<(), &str> {
        let from_sq = mov.from;
        let to_sq = mov.from;
        let Some(piece) = self.board.get_piece_at(from_sq) else {
            return Err("No piece selected");
        };


        if piece.color != self.side_to_move {
            return Err("Move opponent piece");
        }

        if let Some(target_piece) = self.board.get_piece_at(to_sq)
            && target_piece.color == self.side_to_move
        {
            return Err("Cannot capture own piece");
        }

        match piece.piece_type {
            PieceType::Pawn => {
                self.board.remove_piece(piece, from_sq);
                if ChessBoard::PAWN_ATTACKS[mov.from.0 as usize] & self.board.all_pieces
                    == Bitboard::EMPTY
                {
                    self.board.add_piece(piece, to_sq);
                    return Ok(());
                }
                return Err("uh");
            }

            PieceType::Rook => {
                let direction = match mov.from.rank() as isize - mov.to.rank() as isize {
                    ..-1 => 3,
                    1.. => 1,
                    _ => match mov.from.file() as isize - mov.to.file() as isize {
                        ..-1 => 0,
                        1.. => 2,
                        _ => return Err("Invalid move"),
                    },
                };
                if ChessBoard::ROOK_ATTACKS[direction][mov.from.0 as usize] & self.board.all_pieces
            }
        }

        Ok(())
    }

    pub fn make_move(&mut self, mv: &ChessMove) {
        let moving_piece = self
            .board
            .get_piece_at(mv.from)
            .expect("make_move called with no piece at 'from' square");

        let captured_piece = self.board.get_piece_at(mv.to);

        self.board.move_piece(mv.from, mv.to, moving_piece);

        if let Some(promo_piece_type) = mv.promotion {
            self.board.remove_piece(moving_piece, mv.to);
            let new_piece = ChessPiece {
                color: self.side_to_move,
                piece_type: promo_piece_type,
            };
            self.board.add_piece(new_piece, mv.to);
        }

        if moving_piece.piece_type == PieceType::Pawn
            && mv.from.file() != mv.to.file()
            && captured_piece.is_none()
        {
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
                (Color::White, f) if f > mv.from.file() => (ChessSquare::H1, ChessSquare::F1), // Kingside
                (Color::White, _) => (ChessSquare::A1, ChessSquare::D1), // Queenside
                (Color::Black, f) if f > mv.from.file() => (ChessSquare::H8, ChessSquare::F8), // Kingside
                (Color::Black, _) => (ChessSquare::A8, ChessSquare::D8), // Queenside
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

        // Rook moved from original square
        match mv.from {
            ChessSquare::H1 => rights_to_remove |= CastlingRights::WHITE_KINGSIDE,
            ChessSquare::A1 => rights_to_remove |= CastlingRights::WHITE_QUEENSIDE,
            ChessSquare::H8 => rights_to_remove |= CastlingRights::BLACK_KINGSIDE,
            ChessSquare::A8 => rights_to_remove |= CastlingRights::BLACK_QUEENSIDE,
            _ => {}
        }

        // Rook was captured on original square
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

        board.push_str("  a b c d e f g h\n"); // File labels
        print!("{board}");
    }

    // fn compute_position_hash(&self) -> u64 {

    // }
}

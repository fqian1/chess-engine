use super::{Bitboard, ChessPiece, ChessSquare, Color, PieceType};

#[derive(Debug, Clone, Default)]
pub struct ChessBoard {
    pieces: [[Bitboard; 6]; 2],
    white_occupancy: Bitboard,
    black_occupancy: Bitboard,
    all_pieces: Bitboard,
}

const fn piece_type_to_index(pt: PieceType) -> usize {
    pt as usize
}

impl ChessBoard {
    const fn generate_knight_attacks() -> [Bitboard; 64] {
        let mut attacks = [Bitboard::EMPTY; 64];

        let mut i = 0;
        while i < 64 {
            let (x, y) = (i % 8, i / 8);
            let mut current_square_attacks = Bitboard::EMPTY;

            let mut dx: i8 = -2;
            while dx <= 2 {
                let mut dy: i8 = -2;
                while dy <= 2 {
                    if dx.abs() + dy.abs() == 3 {
                        let nx = x as i8 + dx;
                        let ny = y as i8 + dy;

                        if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 {
                            let target_square_index = (nx + ny * 8) as usize;
                            current_square_attacks
                                .set(ChessSquare::new(target_square_index as u8).unwrap());
                        }
                    }
                    dy += 1;
                }
                dx += 1;
            }
            attacks[i] = current_square_attacks;
            i += 1;
        }
        attacks
    }

    const fn generate_king_attacks() -> [Bitboard; 64] {
        let mut attacks = [Bitboard::EMPTY; 64];

        let mut i = 0;
        while i < 64 {
            let x = i % 8;
            let y = i / 8;

            let mut current_square_attacks = Bitboard::EMPTY;

            let mut dx: i8 = -1;
            while dx <= 1 {
                let mut dy: i8 = -1;
                while dy <= 1 {
                    if dx != 0 || dy != 0 {
                        let nx = x as i8 + dx;
                        let ny = y as i8 + dy;

                        if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 {
                            let target_square_index = (nx + ny * 8) as usize;
                            current_square_attacks
                                .set(ChessSquare::new(target_square_index as u8).unwrap());
                        }
                    }
                    dy += 1;
                }
                dx += 1;
            }
            attacks[i] = current_square_attacks;
            i += 1;
        }
        attacks
    }

    const fn generate_pawn_moves() -> [Bitboard; 64] {
        let mut attacks = [Bitboard::EMPTY; 64];

        let mut i = 0;
        while i < 56 {
            let square_bb = 1u64 << i;
            let mut current_square_attacks = 0u64;

            current_square_attacks |= square_bb << 8;
            if i >= 8 && i <= 15 {
                current_square_attacks |= square_bb << 16;
            }

            attacks[i] = Bitboard(current_square_attacks);
            i += 1;
        }
        attacks
    }

    const fn generate_pawn_attacks() -> [Bitboard; 64] {
        let mut attacks = [Bitboard::EMPTY; 64];

        const NOT_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
        const NOT_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;

        let mut i = 0;
        while i < 64 {
            let square_bb = 1u64 << i;
            let mut current_square_attacks = 0u64;

            current_square_attacks |= (square_bb << 7) & NOT_A_FILE;
            current_square_attacks |= (square_bb << 9) & NOT_H_FILE;

            attacks[i] = Bitboard(current_square_attacks);
            i += 1;
        }
        attacks
    }

    pub const fn generate_direction_masks() -> (
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
    ) {
        let mut north = [Bitboard::EMPTY; 64];
        let mut south = [Bitboard::EMPTY; 64];
        let mut east = [Bitboard::EMPTY; 64];
        let mut west = [Bitboard::EMPTY; 64];
        let mut ne = [Bitboard::EMPTY; 64];
        let mut nw = [Bitboard::EMPTY; 64];
        let mut se = [Bitboard::EMPTY; 64];
        let mut sw = [Bitboard::EMPTY; 64];

        let mut i = 0;
        while i < 64 {
            let file = i % 8;
            let rank = i / 8;

            // North
            let mut r = rank + 1;
            while r < 8 {
                north[i].set(ChessSquare((r * 8 + file) as u8));
                r += 1;
            }

            // South
            let mut r = rank as i8 - 1;
            while r >= 0 {
                south[i].set(ChessSquare((r as usize * 8 + file) as u8));
                r -= 1;
            }

            // East
            let mut f = file + 1;
            while f < 8 {
                east[i].set(ChessSquare((rank * 8 + f) as u8));
                f += 1;
            }

            // West
            let mut f = file as i8 - 1;
            while f >= 0 {
                west[i].set(ChessSquare((rank * 8 + f as usize) as u8));
                f -= 1;
            }

            // North-East
            let mut r = rank + 1;
            let mut f = file + 1;
            while r < 8 && f < 8 {
                ne[i].set(ChessSquare((r * 8 + f) as u8));
                r += 1;
                f += 1;
            }

            // North-West
            let mut r = rank + 1;
            let mut f = file as i8 - 1;
            while r < 8 && f >= 0 {
                nw[i].set(ChessSquare((r * 8 + f as usize) as u8));
                r += 1;
                f -= 1;
            }

            // South-East
            let mut r = rank as i8 - 1;
            let mut f = file + 1;
            while r >= 0 && f < 8 {
                se[i].set(ChessSquare((r as usize * 8 + f) as u8));
                r -= 1;
                f += 1;
            }

            // South-West
            let mut r = rank as i8 - 1;
            let mut f = file as i8 - 1;
            while r >= 0 && f >= 0 {
                sw[i].set(ChessSquare((r as usize * 8 + f as usize) as u8));
                r -= 1;
                f -= 1;
            }

            i += 1;
        }

        (north, ne, east, se, south, sw, west, nw)
    }

    pub const KNIGHT_ATTACKS: [Bitboard; 64] = Self::generate_knight_attacks();
    pub const KING_ATTACKS: [Bitboard; 64] = Self::generate_king_attacks();
    pub const PAWN_ATTACKS: [Bitboard; 64] = Self::generate_pawn_attacks();
    pub const RAYS: (
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
        [Bitboard; 64],
    ) = Self::generate_direction_masks(); // 0-7 -> n, ne, e, se, s, sw, w, nw
    pub const PAWN_MOVES: [Bitboard; 64] = Self::generate_pawn_moves();

    pub fn empty() -> Self {
        ChessBoard {
            pieces: [[Bitboard::EMPTY; 6]; 2],
            white_occupancy: Bitboard::EMPTY,
            black_occupancy: Bitboard::EMPTY,
            all_pieces: Bitboard::EMPTY,
        }
    }

    pub fn new() -> Self {
        ChessBoard {
            pieces: [
                [
                    Bitboard::BLACK_PAWNS,
                    Bitboard::WHITE_KNIGHTS,
                    Bitboard::WHITE_BISHOPS,
                    Bitboard::WHITE_ROOKS,
                    Bitboard::WHITE_QUEENS,
                    Bitboard::WHITE_KING,
                ],
                [
                    Bitboard::BLACK_PAWNS,
                    Bitboard::BLACK_KNIGHTS,
                    Bitboard::BLACK_BISHOPS,
                    Bitboard::BLACK_ROOKS,
                    Bitboard::BLACK_QUEENS,
                    Bitboard::BLACK_KING,
                ],
            ],
            white_occupancy: Bitboard::WHITE_OCCUPANCY,
            black_occupancy: Bitboard::WHITE_OCCUPANCY,
            all_pieces: Bitboard::ALL_PIECES,
        }
    }

    pub fn get_piece_bitboard(&self, color: Color, piece_type: PieceType) -> Bitboard {
        self.pieces[color as usize][piece_type as usize]
    }

    pub fn remove_piece(&mut self, piece: ChessPiece, square: ChessSquare) {
        let color_idx = piece.color as usize;
        let piece_idx = piece_type_to_index(piece.piece_type);

        self.pieces[color_idx][piece_idx].clear(square);

        match piece.color {
            Color::White => {
                self.white_occupancy.clear(square);
            }
            Color::Black => {
                self.black_occupancy.clear(square);
            }
        }
        self.all_pieces.clear(square);
    }

    pub fn add_piece(&mut self, piece: ChessPiece, square: ChessSquare) {
        let color_idx = piece.color as usize;
        let piece_idx = piece_type_to_index(piece.piece_type);

        self.pieces[color_idx][piece_idx].set(square);

        match piece.color {
            Color::White => {
                self.white_occupancy.set(square);
            }
            Color::Black => {
                self.black_occupancy.set(square);
            }
        }
        self.all_pieces.set(square);
    }

    pub fn move_piece(&mut self, from_sq: ChessSquare, to_sq: ChessSquare, piece: ChessPiece) {
        self.remove_piece(piece, from_sq);
        self.add_piece(piece, to_sq);
    }

    pub fn get_piece_at(&self, square: ChessSquare) -> Option<ChessPiece> {
        if !square.is_valid() {
            return None;
        }

        let square_bit = square.bitboard();

        if (square_bit & self.all_pieces) == Bitboard::empty() {
            return None;
        }

        let color = if (self.white_occupancy & square_bit) != Bitboard::empty() {
            Color::White
        } else {
            Color::Black
        };
        let color_idx = color as usize;

        for piece_idx in 0..6 {
            if (self.pieces[color_idx][piece_idx].0 & square_bit.0) != 0 {
                let piece_type = PieceType::from_idx(piece_idx);
                if let Some(piece) = piece_type {
                    return Some(ChessPiece {
                        color,
                        piece_type: piece,
                    });
                }
            }
        }
        None
    }

    pub fn display_ascii(&self) -> String {
        let mut board_str = String::new();
        board_str.push_str("  a b c d e f g h\n");
        for r in (0..8).rev() {
            board_str.push_str(&format!("{} ", r + 1));
            for f in 0..8 {
                let square = ChessSquare::from_coords(f, r).unwrap();
                let piece_char = match self.get_piece_at(square) {
                    Some(p) => p.piece_type.to_char(p.color),
                    None => '.',
                };
                board_str.push(piece_char);
                board_str.push(' ');
            }
            board_str.push('\n');
        }
        board_str
    }
}

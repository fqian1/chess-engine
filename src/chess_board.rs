use super::{Bitboard, ChessMove, ChessPiece, ChessSquare, Color, PieceType};

#[derive(Debug, Clone, Default)]
pub struct ChessBoard {
    pub pieces: [[Bitboard; 6]; 2],
    pub white_occupancy: Bitboard,
    pub black_occupancy: Bitboard,
    pub all_pieces: Bitboard,
}

const fn piece_type_to_index(pt: PieceType) -> usize {
    pt as usize
}

impl ChessBoard {
    const fn generate_between() -> [[Option<Bitboard>; 64]; 64] {
        let mut between = [[None; 64]; 64];

        let mut s1 = 0;
        while s1 < 64 {
            let mut s2 = 0;
            while s2 < 64 {
                let r1 = (s1 / 8) as i8;
                let f1 = (s1 % 8) as i8;
                let r2 = (s2 / 8) as i8;
                let f2 = (s2 % 8) as i8;

                let dr = r2 - r1;
                let df = f2 - f1;

                if dr == 0 || df == 0 || dr.abs() == df.abs() {
                    let mut bb = Bitboard::EMPTY;

                    let step_r = dr.signum();
                    let step_f = df.signum();

                    let mut curr_r = r1 + step_r;
                    let mut curr_f = f1 + step_f;

                    while curr_r != r2 || curr_f != f2 {
                        let bit_idx = (curr_r * 8 + curr_f) as u8;
                        bb.set(ChessSquare(bit_idx));

                        curr_r += step_r;
                        curr_f += step_f;
                    }
                    between[s1][s2] = Some(bb);
                }
                s2 += 1;
            }
            s1 += 1;
        }
        between
    }

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
                            current_square_attacks.set(ChessSquare::new(target_square_index as u8).unwrap());
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
                            current_square_attacks.set(ChessSquare::new(target_square_index as u8).unwrap());
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

    const fn generate_white_pawn_attacks() -> [Bitboard; 64] {
        let mut attacks = [Bitboard::EMPTY; 64];

        const NOT_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
        const NOT_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;

        let mut i = 0;
        while i < 64 {
            let square_bb = 1u64 << i;
            let mut current_square_attacks = 0u64;

            current_square_attacks |= (square_bb << 7) & NOT_H_FILE;
            current_square_attacks |= (square_bb << 9) & NOT_A_FILE;

            attacks[i] = Bitboard(current_square_attacks);
            i += 1;
        }
        attacks
    }

    const fn generate_black_pawn_attacks() -> [Bitboard; 64] {
        let mut attacks = [Bitboard::EMPTY; 64];

        const NOT_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
        const NOT_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;

        let mut i = 0;
        while i < 64 {
            let square_bb = 1u64 << i;
            let mut current_square_attacks = 0u64;

            current_square_attacks |= (square_bb >> 7) & NOT_A_FILE;
            current_square_attacks |= (square_bb >> 9) & NOT_H_FILE;

            attacks[i] = Bitboard(current_square_attacks);
            i += 1;
        }
        attacks
    }

    pub fn is_square_attacked(&self, sq: ChessSquare, attacker_color: Color) -> bool {
        let enemy_pieces = &self.pieces[attacker_color as usize];
        let all_pieces = self.all_pieces;
        let bishop_attacks = ChessBoard::BISHOP_ATTACKS.map(|x| x[0] | x[1] | x[2] | x[3]);
        let rook_attacks = ChessBoard::ROOK_ATTACKS.map(|x| x[0] | x[1] | x[2] | x[3]);

        let incoming_pawn_mask = match attacker_color {
            Color::White => ChessBoard::PAWN_ATTACKS_BLACK[sq.0 as usize],
            Color::Black => ChessBoard::PAWN_ATTACKS_WHITE[sq.0 as usize],
        };

        if !(incoming_pawn_mask & self.pieces[attacker_color as usize][PieceType::Pawn as usize]).is_empty() {
            return true;
        }

        if !(ChessBoard::KNIGHT_ATTACKS[sq.0 as usize] & enemy_pieces[PieceType::Knight as usize]).is_empty() {
            return true;
        }

        if !(ChessBoard::KING_ATTACKS[sq.0 as usize] & enemy_pieces[PieceType::King as usize]).is_empty() {
            return true;
        }

        let mut diagonal_attackers = (enemy_pieces[PieceType::Bishop as usize]
            | enemy_pieces[PieceType::Queen as usize])
            & bishop_attacks[sq.0 as usize];

        while let Some(attacker_sq) = diagonal_attackers.pop_lsb() {
            let path = ChessBoard::BETWEEN[sq.0 as usize][attacker_sq.0 as usize].unwrap();
            if (path & all_pieces).is_empty() {
                return true;
            }
        }

        let mut straight_attackers = (enemy_pieces[PieceType::Rook as usize] | enemy_pieces[PieceType::Queen as usize])
            & rook_attacks[sq.0 as usize];

        while let Some(attacker_sq) = straight_attackers.pop_lsb() {
            let path = ChessBoard::BETWEEN[sq.0 as usize][attacker_sq.0 as usize].unwrap();
            if (path & all_pieces).is_empty() {
                return true;
            }
        }

        false
    }

    pub const fn generate_rook_direction_masks() -> [[Bitboard; 4]; 64] {
        let mut i = 0;
        let mut boards = [[Bitboard::EMPTY; 4]; 64];

        while i < 64 {
            let file = i % 8;
            let rank = i / 8;

            let mut r = rank + 1;
            while r < 8 {
                boards[i][0].set(ChessSquare((r * 8 + file) as u8));
                r += 1;
            }

            let mut r = rank as i8 - 1;
            while r >= 0 {
                boards[i][1].set(ChessSquare((r as usize * 8 + file) as u8));
                r -= 1;
            }

            let mut f = file + 1;
            while f < 8 {
                boards[i][2].set(ChessSquare((rank * 8 + f) as u8));
                f += 1;
            }

            let mut f = file as i8 - 1;
            while f >= 0 {
                boards[i][3].set(ChessSquare((rank * 8 + f as usize) as u8));
                f -= 1;
            }

            i += 1;
        }
        boards
    }

    const fn generate_bishop_direction_masks() -> [[Bitboard; 4]; 64] {
        let mut boards = [[Bitboard::EMPTY; 4]; 64];

        let mut i = 0;
        while i < 64 {
            let file = i % 8;
            let rank = i / 8;

            let mut j = 1;
            while file >= j && rank >= j {
                boards[i][0].set(ChessSquare(((rank - j) * 8 + (file - j)) as u8));
                j += 1;
            }

            j = 1;
            while file + j < 8 && rank >= j {
                boards[i][1].set(ChessSquare(((rank - j) * 8 + (file + j)) as u8));
                j += 1;
            }

            j = 1;
            while file + j < 8 && rank + j < 8 {
                boards[i][2].set(ChessSquare(((rank + j) * 8 + (file + j)) as u8));
                j += 1;
            }

            j = 1;
            while file >= j && rank + j < 8 {
                boards[i][3].set(ChessSquare(((rank + j) * 8 + (file - j)) as u8));
                j += 1;
            }

            i += 1;
        }

        boards
    }

    pub const WHITE_SQUARES: Bitboard = Bitboard(0xAA55AA55AA55AA55);
    pub const BLACK_SQUARES: Bitboard = Bitboard(0x55AA55AA55AA55AA);
    pub const KNIGHT_ATTACKS: [Bitboard; 64] = Self::generate_knight_attacks();
    pub const KING_ATTACKS: [Bitboard; 64] = Self::generate_king_attacks();
    pub const PAWN_ATTACKS_WHITE: [Bitboard; 64] = Self::generate_white_pawn_attacks();
    pub const PAWN_ATTACKS_BLACK: [Bitboard; 64] = Self::generate_black_pawn_attacks();
    // NORTH EAST SOUTH WEST
    pub const ROOK_ATTACKS: [[Bitboard; 4]; 64] = Self::generate_rook_direction_masks();
    // NW NE SE SW
    pub const BISHOP_ATTACKS: [[Bitboard; 4]; 64] = Self::generate_bishop_direction_masks();

    pub const BETWEEN: [[Option<Bitboard>; 64]; 64] = Self::generate_between();

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
                    Bitboard::WHITE_PAWNS,
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
        self.white_occupancy.clear(square);
        self.black_occupancy.clear(square);
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

    pub fn flip_board(&self) -> [[Bitboard; 6]; 2] {
        [self.pieces[1].map(|b| b.flipped()), self.pieces[0].map(|b| b.flipped())]
    }

    pub fn apply_move(&mut self, mov: &ChessMove, side_to_move: Color, en_passant_sq: Option<ChessSquare>) {
        let moving_piece = self.get_piece_at(mov.from).expect("No piece selected");
        let is_en_passant = moving_piece.piece_type == PieceType::Pawn && en_passant_sq.is_some_and(|sq| sq == mov.to);

        if is_en_passant {
            let cap_sq = if side_to_move == Color::White {
                ChessSquare(mov.to.0 - 8)
            } else {
                ChessSquare(mov.to.0 + 8)
            };
            let captured_pawn = ChessPiece { color: side_to_move.opposite(), piece_type: PieceType::Pawn };
            self.remove_piece(captured_pawn, cap_sq);
        } else {
            if let Some(cap_piece) = self.get_piece_at(mov.to) {
                self.remove_piece(cap_piece, mov.to);
            }
        }

        self.move_piece(mov.from, mov.to, moving_piece);

        if let Some(promo_type) = mov.promotion {
            self.remove_piece(moving_piece, mov.to);
            self.add_piece(ChessPiece::new(side_to_move, promo_type), mov.to);
        }

        if moving_piece.piece_type == PieceType::King && (mov.from.file() as i8 - mov.to.file() as i8).abs() == 2 {
            let (rook_from, rook_to) = match (side_to_move, mov.to.file()) {
                (Color::White, f) if f > mov.from.file() => (ChessSquare::H1, ChessSquare::F1),
                (Color::White, _) => (ChessSquare::A1, ChessSquare::D1),
                (Color::Black, f) if f > mov.from.file() => (ChessSquare::H8, ChessSquare::F8),
                (Color::Black, _) => (ChessSquare::A8, ChessSquare::D8),
            };
            let rook = ChessPiece::new(side_to_move, PieceType::Rook);
            self.move_piece(rook_from, rook_to, rook);
        }
    }

    pub fn get_piece_at(&self, square: ChessSquare) -> Option<ChessPiece> {
        if !square.is_valid() {
            return None;
        }

        let square_bit = square.bitboard();

        if (square_bit & self.all_pieces) == Bitboard::EMPTY {
            return None;
        }

        let color = if (self.white_occupancy & square_bit) != Bitboard::EMPTY {
            Color::White
        } else {
            Color::Black
        };
        let color_idx = color as usize;

        for piece_idx in 0..6 {
            if (self.pieces[color_idx][piece_idx].0 & square_bit.0) != 0 {
                let piece_type = PieceType::from_idx(piece_idx);
                if let Some(piece) = piece_type {
                    return Some(ChessPiece { color, piece_type: piece });
                }
            }
        }
        None
    }

    pub fn display_ascii(&self) -> String {
        if cfg!(debug_assertions) {
            println!("Printing ascii");
        }
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

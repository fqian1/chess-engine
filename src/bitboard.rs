use super::ChessSquare;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const ALL: Bitboard = Bitboard(u64::MAX);
    pub const WHITE_PAWNS: Bitboard = Bitboard(0x0000_0000_0000_FF00);
    pub const BLACK_PAWNS: Bitboard = Bitboard(0x00FF_0000_0000_0000);
    pub const WHITE_KNIGHTS: Bitboard = Bitboard(0x0000_0000_0000_0042);
    pub const BLACK_KNIGHTS: Bitboard = Bitboard(0x4200_0000_0000_0000);
    pub const WHITE_BISHOPS: Bitboard = Bitboard(0x0000_0000_0000_0024);
    pub const BLACK_BISHOPS: Bitboard = Bitboard(0x2400_0000_0000_0000);
    pub const WHITE_ROOKS: Bitboard = Bitboard(0x0000_0000_0000_0081);
    pub const BLACK_ROOKS: Bitboard = Bitboard(0x8100_0000_0000_0000);
    pub const WHITE_QUEENS: Bitboard = Bitboard(0x0000_0000_0000_0008);
    pub const BLACK_QUEENS: Bitboard = Bitboard(0x0800_0000_0000_0000);
    pub const WHITE_KING: Bitboard = Bitboard(0x0000_0000_0000_0010);
    pub const BLACK_KING: Bitboard = Bitboard(0x1000_0000_0000_0000);

    pub const WHITE_OCCUPANCY: Bitboard = Bitboard(
        Bitboard::WHITE_PAWNS.0
            | Bitboard::WHITE_KNIGHTS.0
            | Bitboard::WHITE_BISHOPS.0
            | Bitboard::WHITE_ROOKS.0
            | Bitboard::WHITE_QUEENS.0
            | Bitboard::WHITE_KING.0,
    );

    pub const BLACK_OCCUPANCY: Bitboard = Bitboard(
        Bitboard::BLACK_PAWNS.0
            | Bitboard::BLACK_KNIGHTS.0
            | Bitboard::BLACK_BISHOPS.0
            | Bitboard::BLACK_ROOKS.0
            | Bitboard::BLACK_QUEENS.0
            | Bitboard::BLACK_KING.0,
    );

    pub const ALL_PIECES: Bitboard =
        Bitboard(Bitboard::BLACK_OCCUPANCY.0 | Bitboard::WHITE_OCCUPANCY.0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn print_bitboard(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = rank * 8 + file;
                if self.0 & (1 << index) != 0 {
                    print!("1 ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    pub fn from_square(square: ChessSquare) -> Self {
        Self(1 << square.index())
    }

    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn is_set(self, square: ChessSquare) -> bool {
        (self.0 & (1u64 << square.0)) != 0
    }

    pub fn lsb_square(self) -> Option<ChessSquare> {
        if self.0 == 0 {
            None
        } else {
            Some(ChessSquare(self.0.trailing_zeros() as u8))
        }
    }

    pub fn lsb_idx(&self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as u8)
        }
    }

    pub fn pop_lsb(&mut self) -> Option<ChessSquare> {
        if let Some(idx) = self.lsb_idx() {
            self.0 ^= 1u64 << idx;
            Some(ChessSquare(idx))
        } else {
            None
        }
    }

    pub fn msb_square(self) -> Option<ChessSquare> {
        if self.0 == 0 {
            None
        } else {
            Some(ChessSquare(63 - self.0.leading_zeros() as u8))
        }
    }

    pub fn contains(&self, square: ChessSquare) -> bool {
        (self.0 & (1 << square.index())) != 0
    }

    pub const fn set(&mut self, square: u8) {
        self.0 |= 1 << square;
    }

    pub const fn clear(&mut self, square: u8) {
        self.0 &= !(1 << square);
    }

    pub const fn toggle(&mut self, square: u8) {
        self.0 ^= 1 << square;
    }

    pub fn union(self, other: Self) -> Self {
        Bitboard(self.0 | other.0)
    }

    pub fn intersection(self, other: Self) -> Self {
        Bitboard(self.0 & other.0)
    }

    pub fn difference(self, other: Self) -> Self {
        Bitboard(self.0 & !other.0)
    }

    pub fn symmetric_difference(self, other: Self) -> Self {
        Bitboard(self.0 ^ other.0)
    }

    pub fn not(self) -> Self {
        Bitboard(!self.0)
    }

    // Shift left (e.g., pawn push) - assumes no wrap around rank 8
    pub fn shift_north(self) -> Self {
        Bitboard(self.0 << 8)
    }
    // Shift right (e.g., pawn capture) - assumes no wrap around file h
    pub fn shift_east(self) -> Self {
        Bitboard(self.0 >> 1)
    }
    // Shift left (e.g., pawn capture) - assumes no wrap around file a
    pub fn shift_west(self) -> Self {
        Bitboard(self.0 << 1)
    }
    // Shift right (e.g., pawn push) - assumes no wrap around rank 1
    pub fn shift_south(self) -> Self {
        Bitboard(self.0 >> 8)
    }

    pub fn print(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = rank * 8 + file;
                if self.0 & (1 << index) != 0 {
                    print!("1 ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  a b c d e f g h")?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let square = ChessSquare::from_coords(file, rank).unwrap();
                if self.is_set(square) {
                    write!(f, "X ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Implement bitwise operations
impl std::ops::BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

// Implement other bitwise operaqtionstions (BitAnd, BitXor, Not, etc.) similarly...

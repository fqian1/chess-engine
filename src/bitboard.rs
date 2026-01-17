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

    const FILE_A: u64 = 0x0101_0101_0101_0101;
    const FILE_H: u64 = 0x8080_8080_8080_8080;

    pub const fn set(&mut self, square: ChessSquare) {
        self.0 |= 1 << square.0;
    }

    pub fn clear(&mut self, square: ChessSquare) {
        self.0 &= !(1 << square.0);
    }

    pub fn toggle(&mut self, square: ChessSquare) {
        self.0 ^= 1 << square.0;
    }

    pub fn from_square(square: ChessSquare) -> Self {
        Self(1 << square.0)
    }

    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn is_set(&self, square: ChessSquare) -> bool {
        (self.0 & (1u64 << square.0)) != 0
    }

    pub fn lsb_square(&self) -> Option<ChessSquare> {
        if self.is_empty() {
            None
        } else {
            Some(ChessSquare(self.0.trailing_zeros() as u8))
        }
    }

    pub fn msb_square(&self) -> Option<ChessSquare> {
        if self.is_empty() {
            None
        } else {
            Some(ChessSquare(self.0.leading_zeros() as u8))
        }
    }

    pub fn pop_msb(&mut self) -> Option<ChessSquare> {
        let square = self.msb_square()?;
        self.0 &= self.0 - 1;
        Some(square)
    }

    pub fn pop_lsb(&mut self) -> Option<ChessSquare> {
        let square = self.lsb_square()?;
        self.0 &= self.0 - 1;
        Some(square)
    }

    pub fn shift_north(self) -> Self {
        Bitboard(self.0 << 8)
    }

    pub fn shift_south(self) -> Self {
        Bitboard(self.0 >> 8)
    }

    pub fn shift_east(self) -> Self {
        Bitboard((self.0 & !Bitboard::FILE_H) << 1)
    }

    pub fn shift_west(self) -> Self {
        Bitboard((self.0 & !Self::FILE_A) >> 1)
    }

    pub fn shift_north_east(self) -> Self {
        Bitboard((self.0 & !Self::FILE_H) << 9)
    }

    pub fn shift_north_west(self) -> Self {
        Bitboard((self.0 & !Self::FILE_A) << 7)
    }

    pub fn shift_south_east(self) -> Self {
        Bitboard((self.0 & !Self::FILE_H) >> 7)
    }

    pub fn shift_south_west(self) -> Self {
        Bitboard((self.0 & !Self::FILE_A) >> 9)
    }

    pub fn flip(&mut self) {
        self.0 = self.0.swap_bytes()
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

impl std::ops::Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

macro_rules! impl_bit_ops {
    ($($trait:ident, $fn:ident, $op:tt),*) => {
        $(
            impl std::ops::$trait for Bitboard {
                type Output = Self;
                fn $fn(self, rhs: Self) -> Self::Output {
                    Self(self.0 $op rhs.0)
                }
            }
        )*
    };
}

macro_rules! impl_bit_assign_ops {
    ($($trait:ident, $fn:ident, $op:tt),*) => {
        $(
            impl std::ops::$trait for Bitboard {
                fn $fn(&mut self, rhs: Self) {
                    self.0 $op rhs.0;
                }
            }
        )*
    };
}

impl_bit_ops! {
    BitAnd, bitand, &,
    BitOr, bitor, |,
    BitXor, bitxor, ^
}

impl_bit_assign_ops! {
    BitAndAssign, bitand_assign, &=,
    BitOrAssign, bitor_assign, |=,
    BitXorAssign, bitxor_assign, ^=
}

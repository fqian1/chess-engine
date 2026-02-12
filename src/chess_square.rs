use super::Bitboard;
use super::Color;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ChessSquare(pub u8);

impl ChessSquare {
    pub const A1: ChessSquare = ChessSquare(0);
    pub const B1: ChessSquare = ChessSquare(1);
    pub const C1: ChessSquare = ChessSquare(2);
    pub const D1: ChessSquare = ChessSquare(3);
    pub const E1: ChessSquare = ChessSquare(4);
    pub const F1: ChessSquare = ChessSquare(5);
    pub const G1: ChessSquare = ChessSquare(6);
    pub const H1: ChessSquare = ChessSquare(7);

    pub const A2: ChessSquare = ChessSquare(8);
    pub const B2: ChessSquare = ChessSquare(9);
    pub const C2: ChessSquare = ChessSquare(10);
    pub const D2: ChessSquare = ChessSquare(11);
    pub const E2: ChessSquare = ChessSquare(12);
    pub const F2: ChessSquare = ChessSquare(13);
    pub const G2: ChessSquare = ChessSquare(14);
    pub const H2: ChessSquare = ChessSquare(15);

    pub const A3: ChessSquare = ChessSquare(16);
    pub const B3: ChessSquare = ChessSquare(17);
    pub const C3: ChessSquare = ChessSquare(18);
    pub const D3: ChessSquare = ChessSquare(19);
    pub const E3: ChessSquare = ChessSquare(20);
    pub const F3: ChessSquare = ChessSquare(21);
    pub const G3: ChessSquare = ChessSquare(22);
    pub const H3: ChessSquare = ChessSquare(23);

    pub const A4: ChessSquare = ChessSquare(24);
    pub const B4: ChessSquare = ChessSquare(25);
    pub const C4: ChessSquare = ChessSquare(26);
    pub const D4: ChessSquare = ChessSquare(27);
    pub const E4: ChessSquare = ChessSquare(28);
    pub const F4: ChessSquare = ChessSquare(29);
    pub const G4: ChessSquare = ChessSquare(30);
    pub const H4: ChessSquare = ChessSquare(31);

    pub const A5: ChessSquare = ChessSquare(32);
    pub const B5: ChessSquare = ChessSquare(33);
    pub const C5: ChessSquare = ChessSquare(34);
    pub const D5: ChessSquare = ChessSquare(35);
    pub const E5: ChessSquare = ChessSquare(36);
    pub const F5: ChessSquare = ChessSquare(37);
    pub const G5: ChessSquare = ChessSquare(38);
    pub const H5: ChessSquare = ChessSquare(39);

    pub const A6: ChessSquare = ChessSquare(40);
    pub const B6: ChessSquare = ChessSquare(41);
    pub const C6: ChessSquare = ChessSquare(42);
    pub const D6: ChessSquare = ChessSquare(43);
    pub const E6: ChessSquare = ChessSquare(44);
    pub const F6: ChessSquare = ChessSquare(45);
    pub const G6: ChessSquare = ChessSquare(46);
    pub const H6: ChessSquare = ChessSquare(47);

    pub const A7: ChessSquare = ChessSquare(48);
    pub const B7: ChessSquare = ChessSquare(49);
    pub const C7: ChessSquare = ChessSquare(50);
    pub const D7: ChessSquare = ChessSquare(51);
    pub const E7: ChessSquare = ChessSquare(52);
    pub const F7: ChessSquare = ChessSquare(53);
    pub const G7: ChessSquare = ChessSquare(54);
    pub const H7: ChessSquare = ChessSquare(55);

    pub const A8: ChessSquare = ChessSquare(56);
    pub const B8: ChessSquare = ChessSquare(57);
    pub const C8: ChessSquare = ChessSquare(58);
    pub const D8: ChessSquare = ChessSquare(59);
    pub const E8: ChessSquare = ChessSquare(60);
    pub const F8: ChessSquare = ChessSquare(61);
    pub const G8: ChessSquare = ChessSquare(62);
    pub const H8: ChessSquare = ChessSquare(63);

    pub const fn new(index: u8) -> Option<Self> {
        if index < 64 { Some(ChessSquare(index)) } else { None }
    }

    pub const fn from_coords(file: u8, rank: u8) -> Option<Self> {
        if file < 8 && rank < 8 {
            Some(ChessSquare(rank * 8 + file))
        } else {
            None
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        if name.len() == 2 {
            let file = name.chars().nth(0)?;
            let rank = name.chars().nth(1)?;

            let file_idx = match file {
                'a'..='h' => file as u8 - b'a',
                _ => return None,
            };
            let rank_idx = match rank {
                '1'..='8' => rank as u8 - b'1',
                _ => return None,
            };

            Self::from_coords(file_idx, rank_idx)
        } else {
            None
        }
    }

    pub fn colour(self) -> Color {
        if self.0.trailing_zeros() % 2 == 0 {
            return Color::White;
        }
        Color::Black
    }

    pub fn to_name(self) -> String {
        let file = (b'a' + self.file()) as char;
        let rank = (b'1' + self.rank()) as char;
        format!("{file}{rank}")
    }

    pub fn is_valid(self) -> bool {
        self.0 < 64
    }

    pub fn bitboard(self) -> Bitboard {
        Bitboard(1u64 << self.0)
    }

    pub fn square_north(self) -> Option<ChessSquare> {
        ChessSquare::new(self.0 + 8)
    }

    pub fn square_south(self) -> Option<ChessSquare> {
        ChessSquare::new(self.0 - 8)
    }

    pub fn index(&self) -> u8 {
        self.0
    }

    pub fn file(&self) -> u8 {
        self.0 % 8
    }

    pub fn rank(&self) -> u8 {
        self.0 / 8
    }

    pub fn name(&self) -> String {
        format!("{}{}", (b'a' + self.file()) as char, self.rank() + 1)
    }
}

impl fmt::Display for ChessSquare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_name())
    }
}

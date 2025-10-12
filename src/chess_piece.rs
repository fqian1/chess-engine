#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(usize)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'w' | 'W' => Some(Color::White),
            'b' | 'B' => Some(Color::Black),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(usize)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    pub fn from_idx(idx: usize) -> Option<PieceType> {
        match idx {
            0 => Some(PieceType::Pawn),
            1 => Some(PieceType::Knight),
            2 => Some(PieceType::Bishop),
            3 => Some(PieceType::Rook),
            4 => Some(PieceType::Queen),
            5 => Some(PieceType::King),
            _ => None,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'p' | 'P' => Some(PieceType::Pawn),
            'n' | 'N' => Some(PieceType::Knight),
            'b' | 'B' => Some(PieceType::Bishop),
            'r' | 'R' => Some(PieceType::Rook),
            'q' | 'Q' => Some(PieceType::Queen),
            'k' | 'K' => Some(PieceType::King),
            _ => None,
        }
    }

    pub fn to_char(&self, color: Color) -> char {
        let lower = match self {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        if color == Color::White {
            lower.to_ascii_uppercase()
        } else {
            lower
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ChessPiece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl ChessPiece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        ChessPiece { color, piece_type }
    }
}

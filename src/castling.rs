#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    pub const WHITE_KINGSIDE: CastlingRights = CastlingRights(0b0001);
    pub const WHITE_QUEENSIDE: CastlingRights = CastlingRights(0b0010);
    pub const BLACK_KINGSIDE: CastlingRights = CastlingRights(0b0100);
    pub const BLACK_QUEENSIDE: CastlingRights = CastlingRights(0b1000);

    pub fn new() -> Self {
        Self(0b1111)
    }

    pub fn empty() -> Self {
        Self(0b0000)
    }

    pub fn from_fen(fen_part: &str) -> Self {
        let mut rights = CastlingRights::empty();
        if fen_part.contains('K') {
            rights |= Self::WHITE_KINGSIDE;
        }
        if fen_part.contains('Q') {
            rights |= Self::WHITE_QUEENSIDE;
        }
        if fen_part.contains('k') {
            rights |= Self::BLACK_KINGSIDE;
        }
        if fen_part.contains('q') {
            rights |= Self::BLACK_QUEENSIDE;
        }
        rights
    }

    pub fn to_fen(&self) -> String {
        let mut s = String::new();
        if self.has(Self::WHITE_KINGSIDE) {
            s.push('K');
        }
        if self.has(Self::WHITE_QUEENSIDE) {
            s.push('Q');
        }
        if self.has(Self::BLACK_KINGSIDE) {
            s.push('k');
        }
        if self.has(Self::BLACK_QUEENSIDE) {
            s.push('q');
        }
        if s.is_empty() { "-".to_string() } else { s }
    }

    pub fn has(&self, right: CastlingRights) -> bool {
        (self.0 & right.0) != 0
    }

    pub fn remove(&mut self, rights_to_remove: CastlingRights) {
        *self &= !rights_to_remove;
    }
}

// Implement bitwise operations
impl std::ops::BitOr for CastlingRights {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOr<u8> for CastlingRights {
    type Output = Self;
    fn bitor(self, rhs: u8) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl std::ops::BitOr<CastlingRights> for u8 {
    type Output = CastlingRights;
    fn bitor(self, rhs: CastlingRights) -> Self::Output {
        CastlingRights(self | rhs.0)
    }
}

impl std::ops::BitOrAssign for CastlingRights {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAndAssign for CastlingRights {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitXor for CastlingRights {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for CastlingRights {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

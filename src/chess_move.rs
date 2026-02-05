use super::{ChessSquare, PieceType};

#[derive(Clone, Debug)]
pub struct ChessMove {
    pub from: ChessSquare,
    pub to: ChessSquare,
    pub promotion: Option<PieceType>,
}

impl ChessMove {
    pub fn new(from: ChessSquare, to: ChessSquare, promotion: Option<PieceType>) -> Self {
        Self { from, to, promotion: promotion }
    }

    pub fn from_uci(uci: &str) -> Result<Self, &'static str> {
        if uci.len() < 4 || uci.len() > 5 {
            return Err("Invalid UCI length");
        }

        let from_sq = ChessSquare::from_name(&uci[0..2]);
        let to_sq = ChessSquare::from_name(&uci[2..4]);

        let promotion = if uci.len() == 5 {
            let promo_char = uci.chars().nth(4).ok_or("Invalid promotion character")?;
            if promo_char != 'Q' && promo_char != 'R' && promo_char != 'B' && promo_char != 'N' {
                return Err("Invalid promotion piece");
            }
            Some(PieceType::from_char(promo_char).ok_or("Invalid promotion piece type")?)
        } else {
            None
        };

        Ok(ChessMove { from: from_sq.unwrap(), to: to_sq.unwrap(), promotion })
    }

    pub fn to_uci(&self) -> String {
        let mut uci = format!("{}{}", self.from.name(), self.to.name());
        if let Some(promotion) = self.promotion {
            uci.push(match promotion {
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                _ => ' ',
            });
        }
        uci
    }
}

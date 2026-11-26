use crate::Serializable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub kind: PieceType,
    pub white: bool,
}

impl Piece {
    fn to_ascii(&self) -> u8 {
        let letter = match &self.kind {
            PieceType::Pawn => b'p',
            PieceType::Knight => b'n',
            PieceType::Bishop => b'b',
            PieceType::Rook => b'r',
            PieceType::Queen => b'q',
            PieceType::King => b'k',
        };

        if self.white {
            letter.to_ascii_uppercase()
        } else {
            letter
        }
    }

    pub fn from_ascii(c: u8) -> Result<Self, ()> {
        let white = c.is_ascii_uppercase();

        let kind = match c.to_ascii_lowercase() {
            b'p' => PieceType::Pawn,
            b'n' => PieceType::Knight,
            b'b' => PieceType::Bishop,
            b'r' => PieceType::Rook,
            b'q' => PieceType::Queen,
            b'k' => PieceType::King,
            _ => return Err(()),
        };

        Ok(Self { kind, white })
    }
}

impl Serializable for Piece{
    fn serialize(&self) -> Vec<u8> {
        vec![self.to_ascii()]
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, ()> {
        match bytes {
            [byte] => Self::from_ascii(*byte),
            _ => Err(()),
        }
    }
    fn size_in_memory() -> usize {1}
}

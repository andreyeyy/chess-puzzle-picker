use crate::Error;
use crate::Serializable;

#[derive(Debug, Clone)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone)]
pub enum Square {
    Empty,
    White(Piece),
    Black(Piece),
}

impl Square {

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    pub fn piece_type(&self) -> Option<&Piece> {
        match self {
            Self::Empty => None,
            Self::White(p) | Self::Black(p) => Some(p),
        }
    }

    fn to_byte(&self) -> u8 {
        match self {
            Square::Empty => b'.',
            Square::White(Piece::Pawn) => b'P',
            Square::White(Piece::Knight) => b'N',
            Square::White(Piece::Bishop) => b'B',
            Square::White(Piece::Rook) => b'R',
            Square::White(Piece::Queen) => b'Q',
            Square::White(Piece::King) => b'K',

            Square::Black(Piece::Pawn) => b'p',
            Square::Black(Piece::Knight) => b'n',
            Square::Black(Piece::Bishop) => b'b',
            Square::Black(Piece::Rook) => b'r',
            Square::Black(Piece::Queen) => b'q',
            Square::Black(Piece::King) => b'k',
        }
    }

    pub fn from_char(c: u8) -> Result<Self, Error> {
        match c {
            b'.' => Ok(Square::Empty),
            b'P' => Ok(Square::White(Piece::Pawn)),
            b'N' => Ok(Square::White(Piece::Knight)),
            b'B' => Ok(Square::White(Piece::Bishop)),
            b'R' => Ok(Square::White(Piece::Rook)),
            b'Q' => Ok(Square::White(Piece::Queen)),
            b'K' => Ok(Square::White(Piece::King)),

            b'p' => Ok(Square::Black(Piece::Pawn)),
            b'n' => Ok(Square::Black(Piece::Knight)),
            b'b' => Ok(Square::Black(Piece::Bishop)),
            b'r' => Ok(Square::Black(Piece::Rook)),
            b'q' => Ok(Square::Black(Piece::Queen)),
            b'k' => Ok(Square::Black(Piece::King)),

            _ => Err(Error::InvalidPieceChar(c as char)),
        }
    }
}

impl Serializable for Square {
    fn serialize(&self) -> Vec<u8> {
        vec![self.to_byte()]
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            [byte] => Square::from_char(*byte),
            _ => Err(Error::DeserializationError),
        }
    }
    fn size_in_memory() -> usize {1}
}

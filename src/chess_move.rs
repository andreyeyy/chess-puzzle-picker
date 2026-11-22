use crate::Error;
use crate::Serializable;
use crate::indexing::*;
use crate::Square;
use crate::Piece;

#[derive(Debug)]
pub enum Promotion {
    Queen,
    Rook,
    Knight,
    Bishop,
}

impl Promotion {
    pub fn from_char(c: u8) -> Result<Self, Error> {
        match c {
            b'q' => Ok(Promotion::Queen),
            b'r' => Ok(Promotion::Rook),
            b'n' => Ok(Promotion::Knight),
            b'b' => Ok(Promotion::Bishop),
            _    => Err(Error::InvalidPromotion(c as char))
        }
    }

    pub fn to_square(&self, color_white: bool) -> Square {
        let piece_type = match self {
            Self::Queen => Piece::Queen,
            Self::Rook => Piece::Rook,
            Self::Knight => Piece::Knight,
            Self::Bishop => Piece::Bishop,
        };

        if color_white {
            Square::White(piece_type)
        }
        else {
            Square::Black(piece_type)
        }
    }
}

impl Serializable for Promotion {

    fn serialize(&self) -> Vec<u8> {
        match self {
            Promotion::Queen => vec![b'q'],
            Promotion::Rook => vec![b'r'],
            Promotion::Knight => vec![b'n'],
            Promotion::Bishop => vec![b'b'],
        }
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        Promotion::from_char(bytes[0])
    }
    fn size_in_memory() -> usize {1}
}

#[derive(Debug)]
pub struct Move {
    pub from_square: u8,
    pub to_square: u8,
    pub promotion: Option<Promotion>,
}

impl Move {

    pub fn from_str(squares: &[u8]) -> Result<Self, Error> {
        let promotion: Option<Promotion>;

        if squares.len() == 4 {
            promotion = None
        }
        else if squares.len() == 5 {
            promotion = Some(Promotion::from_char(squares[4])?);
        }
        else {
            return Err(Error::InvalidStringLength(squares.len()));
        }

        Ok(Move {
            from_square: index_from_ascii(&squares[0..2])?,
            to_square: index_from_ascii(&squares[2..4])?,
            promotion,
        })
    }
}

impl Serializable for Move {

    fn serialize(&self) -> Vec<u8> {

        let promotion_byte = match &self.promotion {
            Some(p) => p.serialize()[0],
            None => 0,
        };

        vec![self.from_square, self.to_square, promotion_byte]
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 3 {
            Err(Error::DeserializationError)
        }
        else {
            let promotion = match bytes[2] {
                0 => None,
                c => Some(Promotion::deserialize(&[c])?),
            };
            Ok(Move {
                from_square: bytes[0],
                to_square: bytes[1],
                promotion,
            })
        }
    }
    fn size_in_memory() -> usize {2 + Promotion::size_in_memory()}
}

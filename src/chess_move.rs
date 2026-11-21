use crate::Error;
use crate::Serializable;
use crate::indexing::*;

#[derive(Debug)]
pub struct Move {
    pub from_square: u8,
    pub to_square: u8,
}

impl Move {

    pub fn from_str(squares: &[u8]) -> Result<Self, Error> {

        if squares.len() != 4 {
            return Err(Error::InvalidStringLength(squares.len()));
        }

        Ok(Move {
            from_square: index_from_ascii(&squares[0..2])?,
            to_square: index_from_ascii(&squares[2..4])?,
        })
    }
}

impl Serializable for Move {
    fn serialize(&self) -> Vec<u8> {
        vec![self.from_square, self.to_square]
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 2 {
            Err(Error::DeserializationError)
        }
        else {
            Ok(Move {
                from_square: bytes[0],
                to_square: bytes[1],
            })
        }
    }
    fn size_in_memory() -> usize {2}
}

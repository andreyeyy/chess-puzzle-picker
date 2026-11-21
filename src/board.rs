use crate::Serializable;
use crate::Error;
use crate::square::Square;
use crate::indexing;

#[derive(Debug)]
pub struct Board {
    pub squares: [Square; 64],
    pub white_to_move: bool,
}

impl Board {
    /// Turns a &str in FEN notation, turns it into a Board
    pub fn parse_fen(fen: &str) -> Result<Self, Error> {
        let mut squares: [Square; 64] = [const {Square::Empty}; 64];
        let white_to_move: bool;

        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        let mut chars = fen.chars();

        for c in chars.by_ref() {

            if c == ' ' {
                break;
            }
            if c.is_ascii_digit() {
                file += c.to_digit(10).ok_or(Error::InvalidFen(fen.to_string()))? as u8;
                continue;
            }
            if c == '/' {
                rank -= 1;
                file = 0;
                continue;
            }
            let index = indexing::index_from_position(rank, file)?;
            if index as usize >= squares.len(){
                return Err(Error::IndexOutOfBounds(index as u8));
            }
            squares[index as usize] = Square::from_char(c as u8)?;
            file += 1;
        }

        match chars.next().ok_or(Error::InvalidFen(fen.to_string()))? {
            'w' => white_to_move = true,
            'b' => white_to_move = false,
            _   =>  return Err(Error::InvalidFen(fen.to_string()))
        }

        Ok(
            Board{
                squares,
                white_to_move,
            }
        )
    }
}


impl Serializable for Board {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::size_in_memory());

        for square in &self.squares {
            result.extend(square.serialize());
        }

        result.push(u8::from(self.white_to_move));

        return result;
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != Self::size_in_memory() {
            return Err(Error::DeserializationError);
        }

        let mut squares: [Square; 64] =
            [const { Square::Empty }; 64];

        for (i, byte) in bytes[..64].iter().enumerate() {
            squares[i] = Square::from_char(*byte)?;
        }

        let white_to_move = bool::try_from(bytes[64]).map_err(|_| Error::DeserializationError)?; 

        Ok(Board {
            squares,
            white_to_move,
        })
    }
    fn size_in_memory() -> usize {65}
}

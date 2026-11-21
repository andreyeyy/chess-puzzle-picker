use crate::{Error, Serializable, Board, Move};

#[derive(Debug)]
pub struct Puzzle {
    pub rating: u16,
    pub board: Board,
    pub solution: Move,
}

impl Puzzle {
    /// Takes a line in the .csv database, turns it into Puzzle
    pub fn from_str(line: &str) -> Result<Self, Error> {

        let mut chars_iter = line.chars();
        let mut fen_raw = String::new();
        let mut solution_raw = String::new();
        let mut rating_raw = String::new();

        chars_iter.find(|&c| c == ',');

        for c in chars_iter.by_ref() {
            if c == ',' {break;}
            fen_raw.push(c);
        }
        chars_iter.find(|&c| c == ' ');
        for _ in 0..4 {
            solution_raw.push(chars_iter.by_ref().next().expect("Expected solution"));
        }

        chars_iter.find(|&c| c == ',');

        for c in chars_iter.by_ref() {
            if c == ',' {break;}
            rating_raw.push(c);
        }

        let rating: u16 = rating_raw.parse().expect("Valid rating");
        let solution = Move::from_str(&solution_raw.into_bytes()).expect("Valid solution");
        let board = Board::parse_fen(&fen_raw).unwrap();

        Ok(Puzzle {
            rating,
            board,
            solution,
        })
    }
}

impl Serializable for Puzzle {
    fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(&self.rating.to_le_bytes());
        result.extend_from_slice(&self.solution.serialize());
        result.extend_from_slice(&self.board.serialize());

        return result;
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != Self::size_in_memory() {
            return Err(Error::DeserializationError);
        }
        let rating = u16::from_le_bytes([bytes[0], bytes[1]]);
        let solution = Move::deserialize(&bytes[2..(2+Move::size_in_memory())])?;
        let board_start_index = 2+Move::size_in_memory();
        let board = Board::deserialize(&bytes[board_start_index..(board_start_index + Board::size_in_memory())])?;

        Ok(Self {
            rating,
            board,
            solution,
        })
    }

    fn size_in_memory() -> usize {
        2 + Board::size_in_memory() + Move::size_in_memory()
    }
}

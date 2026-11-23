use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::{Error, Serializable, Board, Move, indexing};

#[derive(Debug)]
pub enum ValidationResult {
    /// The answer provided by the user is a correct solution, they solved the puzzle.
    CorrectSolution,

    /// The answer provided by the user is wrong, they did not solve the puzzle.
    WrongSolution,

    /// The answer provided is invalid, meaning it cannot possibly be an answer to any puzzle.
    InvalidSolution,
}

#[derive(Debug)]
pub struct Puzzle {
    pub rating: u16,
    pub board: Board,
    pub solution: Move,
}

impl Puzzle {

    /// Checks whether a string is the solution to the puzzle
    pub fn validate_solution(&self, solution: &str) -> ValidationResult {
        if !solution.is_ascii() {
            return ValidationResult::InvalidSolution;
        }

        let normalized = solution.trim().to_ascii_lowercase();
        let bytes = normalized.as_bytes();

        if bytes.len() != 4 && bytes.len() != 5 {
            return ValidationResult::InvalidSolution;
        }

        // Validating origin square
        if indexing::index_from_ascii(&bytes[0..2]).is_err() {
            return ValidationResult::InvalidSolution;
        }

        // Validating destination square
        if indexing::index_from_ascii(&bytes[2..4]).is_err() {
            return ValidationResult::InvalidSolution;
        }

        // Validating promotion character
        if bytes.len() == 5 && !matches!(bytes[4], b'q' | b'r' | b'n' | b'b') {
            return ValidationResult::InvalidSolution;
        }

        // Checking answer
        if normalized == self.solution.to_string() {
            ValidationResult::CorrectSolution
        } else {
            ValidationResult::WrongSolution
        }
    }

    /// Takes a line in the .csv database, turns it into Puzzle
    pub fn from_str(line: &str) -> Result<Self, Error> {
        //PuzzleId,FEN,Moves,Rating,RatingDeviation,Popularity,NbPlays,Themes,GameUrl,OpeningTags,DailyDate
        let fields: Vec<&str> = line.split(',').collect();

        let fen_raw = fields[1];
        let mut moves = fields[2].split_whitespace();
        let last_move_raw = moves.next().ok_or(Error::InvalidCsvRow(line.to_string()))?;
        let solution_raw = moves.next().ok_or(Error::InvalidCsvRow(line.to_string()))?;
        let rating_raw = fields[3];

        let rating: u16 = rating_raw.parse().map_err(|_| Error::InvalidCsvRow(line.to_string()))?;
        let last_move = Move::from_str(&last_move_raw.as_bytes())?;
        let solution = Move::from_str(&solution_raw.as_bytes())?;
        let mut board = Board::parse_fen(fen_raw).unwrap();

        board.apply_move(&last_move)?;

        Ok(Puzzle {
            rating,
            board,
            solution,
        })
    }

    /// Finds the puzzle with the specified ID in the .csv database. Not efficient, 
    /// for debugging purposes only
    pub fn from_csv(path: &Path, id: &str)-> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.contains(id) {
                return Ok(Self::from_str(&line)?);
            }
        }

        panic!()
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

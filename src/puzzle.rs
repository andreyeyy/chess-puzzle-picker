use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::{Serializable, Board, Move, Square};

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
        if Square::from_ascii(&bytes[0..2]).is_err() {
            return ValidationResult::InvalidSolution;
        }

        // Validating destination square
        if Square::from_ascii(&bytes[2..4]).is_err() {
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

    /// Takes a line in the .csv database, turns it into Puzzle.
    /// Returns Err(()) if the line is invalid
    pub fn from_str(line: &str) -> Result<Self, ()> {
        //PuzzleId,FEN,Moves,Rating,RatingDeviation,Popularity,NbPlays,Themes,GameUrl,OpeningTags,DailyDate
        let fields: Vec<&str> = line.split(',').collect();

        let fen_raw = fields[1];
        let mut moves = fields[2].split_whitespace();
        let last_move_raw = moves.next().ok_or(())?;
        let solution_raw = moves.next().ok_or(())?;
        let rating_raw = fields[3];

        let rating: u16 = rating_raw.parse().map_err(|_| ())?;
        // dbg!(last_move_raw);
        let last_move = Move::from_ascii(&last_move_raw.as_bytes())?;
        let solution = Move::from_ascii(&solution_raw.as_bytes())?;
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
    pub fn from_csv(path: &Path, id: &str)-> Result<Self, ()> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
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

    fn deserialize(bytes: &[u8]) -> Result<Self, ()> {
        if bytes.len() != Self::size_in_memory() {
            return Err(());
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




#[cfg(test)]
mod tests {
    use super::*;

    fn sample_puzzle() -> Puzzle {
        let line = "test-id,rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1,e2e4 e7e5,1200";
        Puzzle::from_str(line).expect("valid puzzle data")
    }

    #[test]
    fn accepts_correct_solution_case_insensitively() {
        let puzzle = sample_puzzle();

        assert!(matches!(
            puzzle.validate_solution("E7E5"),
            ValidationResult::CorrectSolution
        ));
    }

    #[test]
    fn rejects_a_valid_but_wrong_move() {
        let puzzle = sample_puzzle();

        assert!(matches!(
            puzzle.validate_solution("d7d5"),
            ValidationResult::WrongSolution
        ));
    }

    #[test]
    fn rejects_malformed_solutions() {
        let puzzle = sample_puzzle();

        for input in ["", "e2", "e7e5x", "é7e5"] {
            assert!(
                matches!(puzzle.validate_solution(input), ValidationResult::InvalidSolution),
                "expected {input:?} to be invalid"
            );
        }
    }
}


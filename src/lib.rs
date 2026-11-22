pub mod square;
pub mod indexing;
pub mod board;
pub mod chess_move;
pub mod puzzle;
pub mod puzzle_database;

#[cfg(feature = "image")]
pub mod render_chessboard;

use board::*;
use chess_move::*;
use puzzle::*;
use puzzle_database::*;

pub const INDEXES_NUM: usize = 30;
pub const INDEXES_DELTA: u16 = 100;

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(bytes: &[u8]) -> Result<Self, Error> where Self: Sized;
    fn size_in_memory() -> usize;
}

#[derive(Debug)]
pub enum Error {
    InvalidMoveChar(char),
    InvalidPieceChar(char),
    InvalidStringLength(usize),
    InvalidPosition((u8, u8)),
    IndexOutOfBounds(u8),
    InvalidFen(String),
    FileReadError,
    FileWriteError,
    DeserializationError,
    InvalidRange,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "something went wrong")
    }
}

impl std::error::Error for Error {}



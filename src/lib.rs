//! # Chess Puzzle Picker
//!
//! This crate lets you efficiently access chess puzzles,
//! stored in a local database built from lichess' puzzle
//! database.
//!
//! - [`PuzzleDatabase`]
//! - [`Puzzle`] 
//!
//! ## Downloading and Building a local database
//! ```
//! use std::path::Path;
//! use chess_puzzle_picker::PuzzleDatabase;
//!
//! fn main() {
//!     let path_to_db = Path::new("/path/to/db.bin");
//!
//!     // Downloads and transforms lichess database, and stores in path_to_db 
//!     PuzzleDatabase::build(&path_to_db).unwrap();
//! }
//! ```
//!
//! ## Building a local database from a .csv file
//! ```
//! use std::path::Path;
//! use chess_puzzle_picker::PuzzleDatabase;
//!
//! fn main() {
//!     let path_to_csv = Path::new("/path/to/lichess_db_puzzle.csv");
//!     let path_to_db = Path::new("/path/to/db.bin");
//!
//!     let db = PuzzleDatabase::from_csv(&path_to_csv).unwrap();
//!     db.write_to_file(&path_to_db);
//! }
//! ```
//!
//! ## Getting a random puzzle
//! ```
//! use std::path::Path;
//! use chess_puzzle_picker::PuzzleDatabase;
//!
//! fn main() {
//!     let path_to_db = Path::new("/path/to/db.bin");
//!     let rating_range = (1700, 2500);
//!
//!     let puzzle = PuzzleDatabase::get_random_puzzle(&path_to_db, rating_range).unwrap();
//! }
//! ```
//!
//! ## Rendering a puzzle as .png
//! This requires the "image" feature to be enabled
//! ```
//! use std::path::Path;
//! use std::io::Write;
//!
//! use chess_puzzle_picker::PuzzleDatabase;
//! use chess_puzzle_picker::render_chessboard::*;
//!
//! fn main() {
//!     let path_to_db = Path::new("/path/to/db.bin");
//!
//!     let rating_range = (1700, 2200);
//!     let puzzle = PuzzleDatabase::get_random_puzzle(path_to_db, rating_range).unwrap();
//!
//!     let style = BoardStyle::default();
//!     let bytes = render_chessboard(&puzzle.board, style).unwrap();
//!
//!     let mut file = std::fs::File::create("/path/to/puzzle.png").unwrap();
//!     file.write_all(&bytes);
//! }
//! ```


pub mod square;
pub mod indexing;
pub mod board;
pub mod chess_move;
pub mod puzzle;
pub mod puzzle_database;

#[cfg(feature = "image")]
pub mod render_chessboard;

#[cfg(feature = "image")]
pub use render_chessboard::render_chessboard;

pub use square::*;
pub use indexing::*;
pub use board::*;
pub use chess_move::*;
pub use puzzle::*;
pub use puzzle_database::*;

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
    InvalidPromotion(char),
    InvalidMove,
    PuzzleNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "something went wrong")
    }
}

impl std::error::Error for Error {}



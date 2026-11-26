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
//!     db.write_to_file(&path_to_db).unwrap();
//! }
//! ```
//!
//! ## Getting a random puzzle
//! ```
//! use std::path::Path;
//! use chess_puzzle_picker::RatingRange;
//! use chess_puzzle_picker::PuzzleDatabase;
//!
//! fn main() {
//!     let path_to_db = Path::new("/path/to/db.bin");
//!     let rating_range = RatingRange::new(1700, 2200).unwrap();
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
//! use chess_puzzle_picker::RatingRange;
//! use chess_puzzle_picker::PuzzleDatabase;
//! use chess_puzzle_picker::render_chessboard::*;
//!
//! fn main() {
//!     let path_to_db = Path::new("/path/to/db.bin");
//!
//!     let rating_range = RatingRange::new(1700, 2200).unwrap();
//!     let puzzle = PuzzleDatabase::get_random_puzzle(path_to_db, rating_range).unwrap();
//!
//!     let style = BoardStyle::default();
//!     let bytes = render_chessboard(&puzzle.board, style).unwrap();
//!
//!     let mut file = std::fs::File::create("/path/to/puzzle.png").unwrap();
//!     file.write_all(&bytes).unwrap();
//! }
//! ```


pub mod square;
pub mod piece;
pub mod board;
pub mod chess_move;
pub mod puzzle;
pub mod puzzle_database;

#[cfg(feature = "image")]
pub mod render_chessboard;

#[cfg(feature = "image")]
pub use render_chessboard::render_chessboard;

pub use square::*;
pub use piece::*;
pub use board::*;
pub use chess_move::*;
pub use puzzle::*;
pub use puzzle_database::*;

pub const INDEXES_NUM: usize = 30;
pub const INDEXES_DELTA: u16 = 100;

/// Lets a type to be losslessly serialized and deserialized
pub trait Serializable {
    /// Encodes the object into a vector of bytes
    fn serialize(&self) -> Vec<u8>;

    /// Decodes the object from a slice of bytes
    fn deserialize(bytes: &[u8]) -> Result<Self, ()> where Self: Sized;

    /// The size of the object in the encoded format
    fn size_in_memory() -> usize;
}

use std::path::Path;
use chess_puzzle_picker::PuzzleDatabase;
use chess_puzzle_picker::Puzzle;
use chess_puzzle_picker::render_chessboard::*;
use std::io::{BufReader, BufRead, Read, BufWriter, Write, Seek, SeekFrom};

fn main() {
    let rating_range = (1700, 2500);
    let path_to_db = Path::new("/home/mint/db.bin");
    let path_to_csv = Path::new("/home/mint/Downloads/lichess_db_puzzle.csv");

    let db = PuzzleDatabase::from_csv(&path_to_csv).unwrap();
    db.write_to_file(&path_to_db);
    let puzzle = PuzzleDatabase::get_random_puzzle(path_to_db, rating_range).unwrap();
    let puzzle = Puzzle::from_csv(path_to_csv, "QKW27").unwrap();

    let style = BoardStyle::default();
    let bytes = render_chessboard(&puzzle.board, style).unwrap();

    let mut file = std::fs::File::create("/home/mint/puzzle.png").unwrap();
    file.write_all(&bytes);
}


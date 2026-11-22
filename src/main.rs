use std::path::Path;
use chess_puzzle_picker::puzzle_database::PuzzleDatabase;
use chess_puzzle_picker::render_chessboard::*;
use std::io::{BufReader, BufRead, Read, BufWriter, Write, Seek, SeekFrom};

fn main() {
    let rating_range = (1700, 2500);
    let path_to_db = Path::new("/home/mint/db.bin");

    let puzzle = PuzzleDatabase::get_random_puzzle(path_to_db, rating_range).unwrap();

    let style = BoardStyle::default();
    let bytes = render_chessboard(&puzzle.board, style).unwrap();

    let mut file = std::fs::File::create("/home/mint/puzzle.png").unwrap();
    file.write_all(&bytes);
}


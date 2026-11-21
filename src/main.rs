use chess_puzzle_picker::*;
use std::path::Path;

fn main() {
    PuzzleDatabase::build(Path::new("/home/mint/db.bin")).unwrap();

    let puzzle = PuzzleDatabase::get_random_puzzle(Path::new("/home/mint/db.bin"), (100, 2500));
    dbg!(puzzle);
}


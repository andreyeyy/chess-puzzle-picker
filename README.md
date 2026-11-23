# Chess Puzzle Picker

This crate lets you efficiently access chess puzzles,
stored in a local database built from the lichess puzzle
database.

## Downloading and Building a local database
```rust
use std::path::Path;
use chess_puzzle_picker::PuzzleDatabase;

fn main() {
    let path_to_db = Path::new("/path/to/db.bin");

    // Downloads and transforms lichess database, and stores in path_to_db 
    PuzzleDatabase::build(&path_to_db).unwrap();
}
```

## Building a local database from a .csv file
```rust
use std::path::Path;
use chess_puzzle_picker::PuzzleDatabase;

fn main() {
    let path_to_csv = Path::new("/path/to/lichess_db_puzzle.csv");
    let path_to_db = Path::new("/path/to/db.bin");

    let db = PuzzleDatabase::from_csv(&path_to_csv).unwrap();
    db.write_to_file(&path_to_db);
}
```

## Getting a random puzzle
```rust
use std::path::Path;
use chess_puzzle_picker::PuzzleDatabase;

fn main() {
    let path_to_db = Path::new("/path/to/db.bin");
    let rating_range = (1700, 2500);

    let puzzle = PuzzleDatabase::get_random_puzzle(&path_to_db, rating_range).unwrap();
}
```

## Rendering a puzzle as .png
This requires the "image" feature to be enabled
```rust
use std::path::Path;
use std::io::Write;

use chess_puzzle_picker::PuzzleDatabase;
use chess_puzzle_picker::render_chessboard::*;

fn main() {
    let path_to_db = Path::new("/path/to/db.bin");

    let rating_range = (1700, 2200);
    let puzzle = PuzzleDatabase::get_random_puzzle(path_to_db, rating_range).unwrap();

    let style = BoardStyle::default();
    let bytes = render_chessboard(&puzzle.board, style).unwrap();

    let mut file = std::fs::File::create("/path/to/puzzle.png").unwrap();
    file.write_all(&bytes);
}
```

use std::path::Path;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead, Read, BufWriter, Write, Seek, SeekFrom};
use crate::{Serializable, Puzzle};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};


fn progress_bar(total_size: Option<u64>) -> ProgressBar {
    let pb = match total_size {
        Some(size) => ProgressBar::new(size),
        None => ProgressBar::new_spinner(),
    };

    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    pb
}

#[derive(Debug)]
pub enum DownloadError {
    Request(reqwest::Error),
    Decode(zstd::Error),
    IO(std::io::Error),
}

#[derive(Debug)]
pub enum BuildError {
    Download(DownloadError),
    Parse(CsvParseError),
    IO(std::io::Error),
}

#[derive(Debug)]
pub enum CsvParseError {
    /// Line number at which parsing error occured
    InvalidLine(u64),

    IO(std::io::Error),
}

#[derive(Debug)]
pub enum PuzzleRetrievalError {
    Deserialization,
    IO(std::io::Error),
}

/// Struct for downloading, transforming and saving the lichess puzzle
/// database for efficient puzzle retrieval
pub struct PuzzleDatabase {
    // indexes_of_ratings[i] is the index at which i*INDEXES_DELTA-rated games start
    puzzles: Vec<Puzzle>,
}

impl PuzzleDatabase {

    /// Downloads and formats the puzzle database, saving the .bin file
    /// to the specified location
    pub fn build(to: &Path) -> Result<(), BuildError> {
        let csv_path = to.with_file_name("lichess_db_puzzle.csv.zst");

        Self::download_and_unpack(&csv_path).map_err(|e| BuildError::Download(e))?;

        let db = Self::from_csv(&csv_path).map_err(|e| BuildError::Parse(e))?;

        println!("Deleting {:?}", &csv_path);
        std::fs::remove_file(&csv_path).map_err(|e| BuildError::IO(e))?;

        db.write_to_file(to).map_err(|e| BuildError::IO(e))?;

        Ok(())
    }

    /// Selects and returns a random puzzle from the .bin database for a specific rating.
    ///
    /// The random sample consists of 'sample_size' puzzles closest to 'rating'.
    ///
    /// If 'sample_size' is larger than the number of puzzles in the database,
    /// a random puzzle from the database is returned.
    pub fn get_random_puzzle(path: &Path, rating: u16, sample_size: u64) -> Result<Puzzle, PuzzleRetrievalError> {
        let half_sample_size = sample_size / 2;

        let total_puzzles = Self::total_puzzles(path).map_err(|e| PuzzleRetrievalError::IO(e))?;
        
        let min_index: u64; let max_index: u64;

        if sample_size >= total_puzzles {
            min_index = 0;
            max_index = total_puzzles - 1;
        }
        else {
            let center_index = Self::find_closest_index(path, rating)?;

            if center_index + half_sample_size >= total_puzzles {
                max_index = total_puzzles - 1;
                min_index = max_index - sample_size;
            }
            else if center_index < half_sample_size {
                min_index = 0;
                max_index = sample_size - 1;
            }
            else {
                min_index = center_index - half_sample_size;
                max_index = center_index + half_sample_size;
            }
        }


        let selected_index = rand::random_range(min_index..max_index);

        Self::get_puzzle(path, selected_index)
    }

    /// Returns the total number of puzzles in the database
    pub fn total_puzzles(path: &Path) -> std::io::Result<u64> {
        Ok (
            std::fs::metadata(path)?.len() / Puzzle::size_in_memory() as u64
        )
    }

    /// Performs binary search on the .bin database
    fn find_closest_index(path: &Path, rating: u16) -> Result<u64, PuzzleRetrievalError> {

        let mut file = File::open(path).map_err(|e| PuzzleRetrievalError::IO(e))?;

        let db_size = file.metadata().map_err(|e| PuzzleRetrievalError::IO(e))?.len();

        let total_indexes: u64 = db_size / Puzzle::size_in_memory() as u64;

        let mut left: u64 = 0;
        let mut right: u64 = total_indexes - 1;

        let mut mid: u64 = 0;

        while left <= right {
            mid = left + (right - left) / 2;
            if mid == 0 {return Ok(0);}

            let puzzle = Self::get_puzzle_from_file(&mut file, mid)?;

            if puzzle.rating == rating {
                return Ok(mid);
            }
            else if puzzle.rating < rating {
                left = mid + 1;
            }
            else {
                right = mid - 1;
            }
        }

        Ok(mid)
    }


    fn get_puzzle_from_file(f: &mut File, index: u64) -> Result<Puzzle, PuzzleRetrievalError> {
        let mut puzzle_bytes: Vec<u8> = vec![0; Puzzle::size_in_memory()];

        let byte_offset = index * Puzzle::size_in_memory() as u64;

        f.seek(SeekFrom::Start(byte_offset)).map_err(|e| PuzzleRetrievalError::IO(e))?;
        f.read_exact(&mut puzzle_bytes).map_err(|e| PuzzleRetrievalError::IO(e))?;

        Puzzle::deserialize(&puzzle_bytes).map_err(|()| PuzzleRetrievalError::Deserialization)
    }

    /// Finds and retrieves the puzzle at 'index' from the .bin database
    pub fn get_puzzle(path: &Path, index: u64) -> Result<Puzzle, PuzzleRetrievalError> {

        let mut file = File::open(path).map_err(|e| PuzzleRetrievalError::IO(e))?;

        Self::get_puzzle_from_file(&mut file, index)
    }


    /// Takes a path to a csv database, parses all puzzles and creates PuzzleDatabase
    pub fn from_csv(path: &Path) -> Result<Self, CsvParseError> {
        let file = File::open(path).map_err(|e| CsvParseError::IO(e))?;
        let reader = BufReader::new(file);

        let mut puzzles: Vec<Puzzle> = Vec::new();

        let file_size: u64 = std::fs::metadata(path).map_err(|e| CsvParseError::IO(e))?.len();
        let mut progress: u64 = 0;

        println!("Reading from {:?}", path);

        let pb = progress_bar(Some(file_size));

        for (n, line) in reader.lines().skip(1).enumerate() {
            let line = line.map_err(|e| CsvParseError::IO(e))?;

            progress += line.bytes().len() as u64;
            pb.set_position(progress);

            let puzzle = Puzzle::from_str(&line).map_err(|()| CsvParseError::InvalidLine(n as u64+1))?;

            puzzles.push(puzzle);
        }
        pb.finish();
        println!("Sorting puzzles");
        puzzles.sort_unstable_by_key(|puzzle| puzzle.rating);

        Ok(PuzzleDatabase {puzzles})
    }

    /// Serializes the puzzle database and writes to a file for future use
    pub fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        println!("Writing to {:?}", path);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let total_size: u64 = (Puzzle::size_in_memory() * self.puzzles.len()) as u64;
        let mut progress: u64 = 0;

        let pb = progress_bar(Some(total_size));

        for puzzle in &self.puzzles {
            let puzzle_bytes = puzzle.serialize();

            progress += puzzle_bytes.len() as u64;
            pb.set_position(progress);

            writer.write_all(&puzzle_bytes)?;
        }
        pb.finish();
        println!("Done!");

        Ok(())
    }

    /// Downloads the puzzle database from lichess.org in the .csv.zst format,
    /// unpacks the archive on the fly and saves the .csv database to the
    /// specified location on the disk.
    pub fn download_and_unpack(to: &Path) -> Result<(), DownloadError> {

        let url = "https://database.lichess.org/lichess_db_puzzle.csv.zst";

        let response = reqwest::blocking::get(url).map_err(|e| DownloadError::Request(e))?
            .error_for_status().map_err(|e| DownloadError::Request(e))?;

        let pb = progress_bar(response.content_length());

        println!("Downloading and unpacking from {url}");

        let tracked_response = pb.wrap_read(response);

        let mut decoder = zstd::stream::read::Decoder::new(tracked_response)
            .map_err(|e| DownloadError::IO(e))?;

        let mut file = File::create(to).map_err(|e| DownloadError::IO(e))?;

        io::copy(&mut decoder, &mut file).map_err(|e| DownloadError::IO(e))?;

        pb.finish();
        Ok(())
    }

}


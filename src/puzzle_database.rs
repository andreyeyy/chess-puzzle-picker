use std::path::Path;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead, Read, BufWriter, Write, Seek, SeekFrom};
use crate::{Serializable, Puzzle, INDEXES_NUM, INDEXES_DELTA};
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

#[derive(Clone)]
pub struct RatingRange {
    min: u16,
    max: u16,
}

impl RatingRange {
    pub fn new(min: u16, max: u16) -> Result<Self, ()> {
        if max < min {
            Err(())
        }
        else {
            Ok(RatingRange {min, max})
        }
    }
    fn indexes(&mut self) -> (usize, usize){

        if self.max > INDEXES_NUM as u16 * INDEXES_DELTA {
            self.max = INDEXES_NUM as u16 * INDEXES_DELTA;
        }

        let min_index = self.min / INDEXES_DELTA;
        let max_index = self.max / INDEXES_DELTA;

        (min_index as usize, max_index as usize)
    }
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
    indexes_of_ratings: [u64; INDEXES_NUM],
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

    /// Selects and returns a random puzzle from the .bin database for a specific rating range.
    pub fn get_random_puzzle(path: &Path, rating_range: RatingRange) -> Result<Puzzle, PuzzleRetrievalError> {
        let mut rating_range = rating_range.clone();

        let (min_index, max_index) = rating_range.indexes();

        let indexes = Self::retrieve_indexes(path).map_err(|e| PuzzleRetrievalError::IO(e))?;

        let start_puzzle_index = indexes[min_index];
        let end_puzzle_index = indexes[max_index];

        let selected_index = rand::random_range(start_puzzle_index..end_puzzle_index);

        Self::get_puzzle(path, selected_index)
    }

    /// Finds and retrieves the puzzle at 'index' from the .bin database
    pub fn get_puzzle(path: &Path, index: u64) -> Result<Puzzle, PuzzleRetrievalError> {

        let byte_offset = index * Puzzle::size_in_memory() as u64 + INDEXES_NUM as u64 * 8;

        let mut file = File::open(path).map_err(|e| PuzzleRetrievalError::IO(e))?;
        file.seek(SeekFrom::Start(byte_offset)).map_err(|e| PuzzleRetrievalError::IO(e))?;

        let mut puzzle_bytes: Vec<u8> = vec![0; Puzzle::size_in_memory()];

        file.read_exact(&mut puzzle_bytes).map_err(|e| PuzzleRetrievalError::IO(e))?;

        let puzzle = Puzzle::deserialize(&puzzle_bytes).map_err(|()| PuzzleRetrievalError::Deserialization)?;

        Ok(puzzle)
    }


    /// Takes a path to a csv database, parses all puzzles and creates PuzzleDatabase
    pub fn from_csv(path: &Path) -> Result<Self, CsvParseError> {
        let file = File::open(path).map_err(|e| CsvParseError::IO(e))?;
        let reader = BufReader::new(file);

        let mut puzzles: Vec<Puzzle> = Vec::new();
        let mut indexes_of_ratings: [u64; INDEXES_NUM] = [0; INDEXES_NUM];

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

        println!("Finding indexes");
        for i in 0..indexes_of_ratings.len() {
            let rating = (i * INDEXES_DELTA as usize) as u16;
            match puzzles.binary_search_by_key(&rating, |x| x.rating) {
                Ok(index) => indexes_of_ratings[i] = index as u64,
                Err(index) => indexes_of_ratings[i] = index as u64,
            }
        }

        Ok(PuzzleDatabase {
            indexes_of_ratings,
            puzzles,
        })
    }

    /// Serializes the puzzle database and writes to a file for future use
    pub fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        println!("Writing to {:?}", path);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        for i in self.indexes_of_ratings {
            writer.write(&i.to_le_bytes())?;
        }

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

    /// Reads and returns indexes from the .bin file
    pub fn retrieve_indexes(path: &Path) -> std::io::Result<[u64; INDEXES_NUM]> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut bytes = [0u8; INDEXES_NUM * 8];
        reader.read_exact(&mut bytes)?;

        let mut indexes: [u64; INDEXES_NUM] = [0; INDEXES_NUM];

        for (value, chunk) in indexes.iter_mut().zip(bytes.chunks_exact(8)) {
            *value = u64::from_le_bytes(chunk.try_into().unwrap());
        }

        Ok(indexes)
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


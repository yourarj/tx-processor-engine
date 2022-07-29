#![cfg(feature = "csv_sync")]

use csv::{Reader, Trim};
use std::io::BufReader;
use std::{error::Error, fs::File};

/// Parse the given file and get the csv reader
///
/// ## Arguments
///
/// * `file` - file path
///
/// ## Returns
///
/// `Result<Reader<BufReader<File>>, Box<dyn Error>>`
pub fn parse(file: String, unsafe_mode: bool) -> Result<Reader<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(file)?;

    let buffer_capacity = 1024 * 1024 * 1024;

    let buf_reader = BufReader::with_capacity(buffer_capacity, file);

    let mut csv_reader_builder = csv::ReaderBuilder::new();
    csv_reader_builder.buffer_capacity(buffer_capacity);

    // if csv input is ill formatted
    // it'll result in performance hit
    // if we are performing additional checks
    if !unsafe_mode {
        csv_reader_builder.trim(Trim::All).flexible(true);
    }

    Ok(csv_reader_builder.from_reader(buf_reader))
}

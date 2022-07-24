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
pub fn parse(file: String) -> Result<Reader<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(file)?;
    let buf_reader = BufReader::with_capacity(512 * 1024 * 1024, file);
    let csv_reader = csv::ReaderBuilder::new()
        .buffer_capacity(512 * 1024 * 1024)
        .trim(Trim::All)
        .flexible(true)
        .from_reader(buf_reader);

    Ok(csv_reader)
}

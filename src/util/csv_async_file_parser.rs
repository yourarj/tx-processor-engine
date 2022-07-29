#![cfg(feature = "csv_async")]

use csv_async::{AsyncDeserializer, AsyncReaderBuilder, Trim};
use std::error::Error;
use tokio::fs::File;

/// Parse the given file and get the csv reader
///
/// ## Arguments
///
/// * `file` - file path
///
/// ## Returns
///
/// `Result<Reader<BufReader<File>>, Box<dyn Error>>`
pub async fn parse(file: String) -> Result<AsyncDeserializer<File>, Box<dyn Error>> {
    let file = File::open(file).await?;

    let csv_reader = AsyncReaderBuilder::new()
        .buffer_capacity(512 * 1024 * 1024)
        .trim(Trim::All)
        .flexible(true)
        .create_deserializer(file);

    Ok(csv_reader)
}

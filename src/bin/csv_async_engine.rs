use clap::Parser;
use std::{collections::HashMap, error::Error};
use tokio_stream::StreamExt;
use tx_processing_engine::{
    client::client::Client,
    tx::{execution_engine::Engine, transaction::Transaction},
    util::args::Args,
    util::csv_async_file_parser::parse,
    util::csv_async_output_writer,
};

/// main method processing file
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file = Args::parse().file;
    let should_log_errors = Args::parse().log_errors;
    let account_state = process_file(file, should_log_errors).await?;
    csv_async_output_writer::write_csv_to_console(&account_state).await?;
    Ok(())
}

/// file processing
/// input: String: filepath
/// input: bool: should print error
async fn process_file(
    file: String,
    log_errors: bool,
) -> Result<HashMap<u16, Client>, Box<dyn Error>> {
    let mut exec_engine = Engine::initialize();
    // TODO handle deserialization error for a malformed record
    let mut async_deserializer = parse(file).await?;
    let mut records = async_deserializer.deserialize::<Transaction>();
    while let Some(result) = records.next().await {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        match result {
            Ok(tx) => match exec_engine.execute_transaction(tx) {
                Ok(_) => (),
                Err(err) => {
                    if log_errors {
                        eprintln!("{}", err);
                    }
                }
            },
            Err(err) => {
                if log_errors {
                    eprintln!("skipped invalid TX: {}", err);
                }
            }
        }
    }
    Ok(exec_engine.get_account_state_owned())
}

use clap::Parser;
use std::error::Error;
use tx_processing_engine::{
    tx::{execution_engine::Engine, transaction::Transaction},
    util::args::Args,
    util::csv_output_writer,
    util::file_parser::parse,
};

/// main method processing file
fn main() -> Result<(), Box<dyn Error>> {
    let file = Args::parse().file;
    let should_log_errors = Args::parse().log_errors;
    process_file(file, should_log_errors)?;
    Ok(())
}

fn process_file(file: String, log_errors: bool) -> Result<(), Box<dyn Error>> {
    let mut exec_engine = Engine::initialize();
    // TODO handle deserialization error for a malformed record
    for result in parse(file)?.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let tx: Transaction = result?;
        match exec_engine.execute_transaction(tx) {
            Ok(_) => (),
            Err(err) => {
                if log_errors {
                    eprintln!("{}", err);
                }
            }
        }
    }
    csv_output_writer::write_csv_to_console(exec_engine.get_account_state())?;
    Ok(())
}

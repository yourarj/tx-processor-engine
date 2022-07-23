use clap::Parser;

/// Arguments we are going accept
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// file to process
    file: String,
}

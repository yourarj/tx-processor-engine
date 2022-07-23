use clap::Parser;

/// Arguments we are going accept
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// file to process
    pub file: String,
    /// should log errors?
    #[clap(short, long, value_parser, default_value_t = false)]
    pub log_errors: bool
}

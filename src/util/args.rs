use clap::Parser;

/// Arguments we are going accept
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// path of file to process
    pub file: String,

    /// Enable error logging [default: false]
    #[clap(short, long, value_parser, default_value_t = false)]
    pub log_errors: bool,

    /// Run in unsafe mode?, considers input csv is sanitized for spaces and column length. trade off between flexibility and performance. [default: false]
    #[clap(short, long, value_parser, default_value_t = false)]
    pub unsafe_mode: bool,
}

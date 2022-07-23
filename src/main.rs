mod command_line_args;

use clap::Parser;
use command_line_args::Args;

/// main method processing file
fn main() {
   let args = Args::parse();

   println!("{:?}", args);
}

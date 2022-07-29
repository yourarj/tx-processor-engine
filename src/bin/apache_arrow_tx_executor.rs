use arrow::csv;
use arrow::datatypes::{DataType, Field, Schema};
use clap::Parser;
use std::fs::File;
use std::sync::Arc;
use tx_processing_engine::util::args::Args;
fn main() {
    let args = Args::parse();

    let schema = Schema::new(vec![
        Field::new("type", DataType::Utf8, false),
        Field::new("client", DataType::UInt16, false),
        Field::new("tx", DataType::UInt32, false),
        Field::new("amount", DataType::Float64, true),
    ]);

    let file = File::open(args.file).unwrap();

    let mut row_counter = 0;

    let mut csv = csv::Reader::new(file, Arc::new(schema), true, None, 10240, None, None, None);
    while let Some(Ok(batch)) = csv.next() {
        row_counter += batch.num_rows()
    }

    println!("Num of rows are: {:08}", row_counter);
}

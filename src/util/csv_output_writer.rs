#![cfg(feature = "csv_sync")]

use std::collections::HashMap;
use std::error::Error;
use std::io::{stdout, BufWriter};

use crate::client::client::Client;
/// Write account status to console
///
/// ## Arguments
///
/// * `clients` - map of client_id and client
///
pub fn write_csv_to_console(clients: &HashMap<u16, Client>) -> Result<(), Box<dyn Error>> {
    let buf_writer = BufWriter::with_capacity(512 * 1024 * 1024, stdout());
    let mut csv_writer = csv::WriterBuilder::new()
        .buffer_capacity(512 * 1024 * 1024)
        .from_writer(buf_writer);
    let clients: Vec<&Client> = clients.values().collect();

    for client in clients {
        csv_writer.serialize(client)?;
    }
    Ok(())
}

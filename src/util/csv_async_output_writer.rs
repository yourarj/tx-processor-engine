#![cfg(feature = "csv_async")]

use std::collections::HashMap;
use std::error::Error;
use tokio::io::stdout;

use csv_async::AsyncWriterBuilder;

use crate::client::client::Client;
/// Write account status to console
///
/// ## Arguments
///
/// * `clients` - map of client_id and client
///
pub async fn write_csv_to_console(clients: &HashMap<u16, Client>) -> Result<(), Box<dyn Error>> {
    let mut csv_writer = AsyncWriterBuilder::new()
        .buffer_capacity(512 * 1024 * 1024)
        .create_serializer(stdout());
    let clients: Vec<&Client> = clients.values().collect();

    for client in clients {
        csv_writer.serialize(client).await?;
    }
    Ok(())
}

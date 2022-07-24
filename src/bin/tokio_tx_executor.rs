///! Should be ignore for now
///! incomplete

use std::{collections::HashMap, error::Error};

use clap::Parser;
use tokio::sync::{broadcast, mpsc};
use tx_processing_engine::{
    client::client::Client,
    tx::{execution_engine::Engine, transaction::Transaction},
    util::csv_output_writer,
    util::{args::Args, file_parser::parse},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let max_threads_supported = num_cpus::get();
    let processor_count = u16::try_from(max_threads_supported).unwrap_or(16);

    let mut account_state: HashMap<u16, Client> = HashMap::new();

    let (result_sender, mut result_receiver) =
        mpsc::channel::<HashMap<u16, Client>>(max_threads_supported);

    let (emitter, subscriber) = broadcast::channel::<Transaction>(4 * max_threads_supported);

    let emitter_clone = emitter.clone();
    let mut handles = Vec::new();
    let file_handle = tokio::spawn(async move {
        let file = Args::parse().file;
        for result in parse(file).expect("unable to open file ").deserialize() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            match result {
                Ok(tx) => {
                    emitter_clone.send(tx).expect("unable to send msg");
                }
                _ => (),
            }
        }
    });
    handles.push(file_handle);

    for task_counter in 0..processor_count {
        let result_sender_clone = result_sender.clone();
        let mut receiver = emitter.subscribe();

        let handle = tokio::spawn(async move {
            // state of task
            let should_log_errors = Args::parse().log_errors;
            // unique id for receiver
            let receiver_id = task_counter;
            // channels listening from

            // task native engine instance
            let mut exec_engine = Engine::initialize();

            while let Ok(tx) = receiver.recv().await {
                if tx.get_client_id() % processor_count == receiver_id {
                    // if this transaction doesn't belong to the current shard just return
                    match exec_engine.execute_transaction(tx) {
                        Ok(_) => (),
                        Err(err) => {
                            if should_log_errors {
                                eprintln!("{}", err);
                            }
                        }
                    }
                }
            }
            result_sender_clone
                .send(exec_engine.get_account_state_owned())
                .await
                .map(|_| println!("result sent"))
                .map_err(|err| eprintln!("{}", err))
                .expect("Error occurred while sending computation result back")
        });
        handles.push(handle);
    }

    let rec_handle = tokio::spawn(async move {
        while let Some(result) = result_receiver.recv().await {
            account_state.extend(result.into_iter());
        }
        csv_output_writer::write_csv_to_console(&account_state).unwrap();
    });
    // drop extra emitter
    drop(emitter);
    // drop the extra subscriber
    drop(subscriber);
    // drop the extra result sender
    drop(result_sender);

    for handle in handles {
        tokio::join!(handle).0.unwrap();
    }

    tokio::join!(rec_handle).0.unwrap();
    Ok(())
}

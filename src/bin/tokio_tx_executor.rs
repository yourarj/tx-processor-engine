///! Should be ignore for now
///! incomplete
use std::{collections::HashMap, error::Error};

use clap::Parser;
use tokio::sync::mpsc::{self, Sender};
use tx_processing_engine::{
    client::client::Client,
    tx::{execution_engine::Engine, transaction::Transaction},
    util::csv_output_writer,
    util::{args::Args, file_parser::parse},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // application args
    let args = Args::parse();
    // detect max threads supported
    let max_threads_supported = num_cpus::get();
    // how many processor to spawn
    let processor_count = u16::try_from(max_threads_supported).unwrap_or(16);

    let mut account_state: HashMap<u16, Client> = HashMap::new();

    let (result_sender, mut result_receiver) =
        mpsc::channel::<HashMap<u16, Client>>(max_threads_supported);

    // repository of sender for each shard
    let mut sender_repository: HashMap<u16, Sender<Transaction>> = HashMap::new();

    for task_counter in 0..processor_count {
        let result_sender_clone = result_sender.clone();
        // create separate mpsc channel for each processor
        let (sender, mut receiver) = mpsc::channel::<Transaction>(10_000);
        sender_repository.insert(task_counter, sender);

        // tx engine instance per shard
        tokio::spawn(async move {
            // unique id for receiver

            // task native engine instance
            let mut exec_engine = Engine::initialize();

            while let Some(tx) = receiver.recv().await {
                if let Err(err) = exec_engine.execute_transaction(tx) {
                    if args.log_errors {
                        eprintln!("{}", err);
                    }
                }
            }
            result_sender_clone
                .send(exec_engine.get_account_state_owned())
                .await
                .map(|_| {
                    println!(
                        "processor: sending computed shard result: {:?}",
                        exec_engine.get_account_state_owned()
                    )
                })
                .map_err(|err| eprintln!("{}", err))
                .expect("Error occurred while sending computation result back")
        });
    }
    drop(result_sender);

    match parse(args.file, args.unsafe_mode) {
        Ok(mut reader) => {
            tokio::spawn(async move {
                for result in reader.deserialize::<Transaction>() {
                    // The iterator yields Result<StringRecord, Error>, so we check the
                    // error here.
                    match result {
                        Ok(tx) => {
                            match sender_repository.get(&(&tx.get_client_id() % processor_count)) {
                                Some(sender) => {
                                    sender.send(tx).await.unwrap_or_else(|_| {
                                        if args.log_errors {
                                            eprintln!("sending failed")
                                        }
                                    });
                                }
                                None => {
                                    if args.log_errors {
                                        eprintln!("Sender not found");
                                    }
                                }
                            }
                        }
                        _ => {
                            if args.log_errors {
                                eprintln!("malformed entry found skipping");
                            }
                        }
                    }
                }
            });
        }
        Err(_) => (),
    }

    while let Some(result) = result_receiver.recv().await {
        println!("Got result: {:?}", &result);
        account_state.extend(result.into_iter());
    }
    csv_output_writer::write_csv_to_console(&account_state).unwrap_or_else(|_| {
        if args.log_errors {
            eprintln!("error while writing output to console");
        }
    });

    Ok(())
}

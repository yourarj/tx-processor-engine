use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use clap::Parser;
use tx_processing_engine::{
    client::client::Client,
    tx::{execution_engine::Engine, transaction::Transaction},
    util::{args::Args, csv_output_writer, file_parser},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_threads_supported = num_cpus::get();
    let args = Args::parse();
    let account_state: Arc<Mutex<HashMap<u16, Client>>> = Arc::new(Mutex::new(HashMap::new()));

    rayon::scope(|s| {
        let (work_queue_sender, work_queue_receiver) = crossbeam_channel::bounded(1000);
        for _task_counter in 0..max_threads_supported * 4 {
            let acc = account_state.clone();
            let work_receiver = work_queue_receiver.clone();
            s.spawn(move |_| {
                let mut exec_engine = Engine::initialize();

                while let Ok(tx) = work_receiver.recv() {
                    if let Err(err) = exec_engine.execute_transaction(tx) {
                        if args.log_errors {
                            eprintln!("{}", err)
                        }
                    }
                }
                acc.lock()
                    .unwrap()
                    .extend(exec_engine.get_account_state_owned().into_iter());
            });
        }

        match file_parser::parse(args.file, args.unsafe_mode) {
            Ok(mut reader) => {
                for result in reader.deserialize::<Transaction>() {
                    match result {
                        Ok(tx) => {
                            work_queue_sender.send(tx).unwrap();
                        }
                        Err(_) => (),
                    }
                }
            }
            Err(_) => (),
        }

        drop(work_queue_sender);
    });

    csv_output_writer::write_csv_to_console(&account_state.lock().unwrap()).unwrap_or_else(|_| {
        if args.log_errors {
            eprintln!("error while writing output to console");
        }
    });

    Ok(())
}

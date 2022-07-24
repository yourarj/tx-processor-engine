//! An awesome 😎 Transaction processor engine
//!
//! exposes
//!
//! * [Core Transaction Execution Engine][engine]
//! * [Transaction][tx::transaction::Transaction]
//! * [utils][util]
//!
//! [engine]: crate::tx::execution_engine::Engine
//!
//! # Examples
//! ```
//! use tx_processing_engine::tx::transaction::Transaction;
//! use tx_processing_engine::tx::execution_engine::Engine;
//! use tx_processing_engine::util::csv_output_writer;
//! use std::error::Error;
//!
//! fn execute_transaction() -> Result<(), Box<dyn Error>>{
//!
//!     // initialize engine
//!     let mut exec_engine = Engine::initialize();
//!
//!     // get tx from source
//!     let tx = Transaction::default();
//!
//!     // execute tx and match output
//!     match exec_engine.execute_transaction(tx) {
//!          Ok(_) => (),
//!          Err(err) => {
//!              // perform error handling here
//!          }
//!      }
//!
//!     // write output to console
//!     csv_output_writer::write_csv_to_console(exec_engine.get_account_state())?;
//!    Ok(())
//! }
//! ```

pub mod client;
pub mod tx;
pub mod util;

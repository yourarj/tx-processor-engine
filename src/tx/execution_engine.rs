use std::{collections::HashMap, error::Error};

use crate::client::client::Client;

use super::{error::TransactionError, transaction::Transaction, transaction_type::TransactionType};

/// transaction execution engine
///
/// holds state of accounts and transactions
pub struct Engine {
    accounts: HashMap<u16, Client>,
    transactions: HashMap<u32, Transaction>,
}

impl Engine {
    /// initialize engine
    pub fn initialize() -> Self {
        Engine {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    /// get engine's current account state
    pub fn get_account_state(&self) -> &HashMap<u16, Client> {
        &self.accounts
    }

    /// get owned copy of account state
    pub fn get_account_state_owned(&self) -> HashMap<u16, Client> {
        self.accounts.clone()
    }

    /// execute the transaction and mutate the engine state accordingly
    ///
    /// ## Arguments
    ///
    /// * `tx` - transaction
    pub fn execute_transaction(&mut self, tx: Transaction) -> Result<(), Box<dyn Error>> {
        // validate transaction amount first
        Self::validate_transaction(&tx)?;

        let tx_id = tx.get_id();
        let tx_type = tx.get_type();
        let tx_amt = tx.get_amt();
        let client_id = tx.get_client_id();

        let opt_tx = self.find_transaction(tx_id);
        let client = self.accounts.get_mut(&client_id);
        let default_client = Client::new(client_id);

        let client = match client {
            Some(client) => client,
            None => {
                self.accounts.insert(client_id, default_client);
                self.accounts.get_mut(&client_id).unwrap()
            }
        };

        match tx_type {
            TransactionType::deposit => {
                client.respond_to_deposit(tx_amt)?;
                self.transactions.insert(tx_id, tx);
                Ok(())
            }
            TransactionType::withdrawal => {
                client.respond_to_withdraw(tx_amt)?;
                self.transactions.insert(tx_id, tx);
                Ok(())
            }
            TransactionType::dispute => {
                let mut inner = opt_tx?;
                Self::is_deposit(&inner)?;
                inner.is_untouched()?;
                client.respond_to_dispute(inner.get_amt())?;
                inner.mark_under_dispute()?;
                self.transactions.insert(inner.get_id(), inner);
                Ok(())
            }
            TransactionType::resolve => {
                let mut inner = opt_tx?;
                inner.is_under_dispute()?;
                client.respond_to_resolve(inner.get_amt())?;
                inner.mark_resolved()?;
                self.transactions.insert(inner.get_id(), inner);
                Ok(())
            }
            TransactionType::chargeback => {
                let mut inner = opt_tx?;
                inner.is_under_dispute()?;
                client.respond_to_chargeback(inner.get_amt())?;
                inner.mark_chargebacked()?;
                self.transactions.insert(inner.get_id(), inner);
                Ok(())
            }
        }
    }

    fn find_transaction(&self, transaction_id: u32) -> Result<Transaction, TransactionError> {
        let tx = self
            .transactions
            .get(&transaction_id)
            .ok_or(TransactionError::TransactionNotFound(transaction_id))?;
        Ok(tx.clone())
    }

    /// validate transaction amount
    ///
    /// ## Arguments
    ///
    /// * `tx` - transaction
    fn validate_transaction(tx: &Transaction) -> Result<(), TransactionError> {
        match tx.get_type() {
            TransactionType::deposit | TransactionType::withdrawal => {
                if tx.get_amt() < 0.0_f64 || tx.get_amt().is_infinite() {
                    Err(TransactionError::InvalidTransactionAmount(
                        tx.get_id(),
                        tx.get_amt(),
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    /// validate transaction is deposit
    ///
    /// ## Arguments
    ///
    /// * `tx` - transaction
    fn is_deposit(tx: &Transaction) -> Result<(), TransactionError> {
        match tx.get_type() {
            TransactionType::deposit => Ok(()),

            _ => Err(TransactionError::InvalidTransactionAmount(
                tx.get_id(),
                tx.get_amt(),
            )),
        }
    }
}

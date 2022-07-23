use std::{collections::HashMap, error::Error};

use crate::client::client::Client;

use super::{error::TransactionError, transaction::Transaction};

pub struct Engine {
    accounts: HashMap<u16, Client>,
    transactions: HashMap<u32, Transaction>,
}

impl Engine {
    pub fn initialize() -> Self {
        Engine {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn get_account_state(&self) -> &HashMap<u16, Client>{
        &self.accounts
    }

    pub fn execute_transaction(&mut self, tx: Transaction) -> Result<(), Box<dyn Error>> {
        let tx_id = tx.get_id();
        let tx_type = tx.get_type();
        let tx_amt = tx.get_amt();
        let client_id = tx.get_client_id();

        let opt_tx = self.find_transaction(tx_id);
        let client = self.accounts.get_mut(&client_id);
        let default_client = Client::new(client_id);

        let client = match client {
            Some(client) => {client},
            None=> {
                self.accounts.insert(client_id, default_client);
                self.accounts.get_mut(&client_id).unwrap()
            }
        };
        

        match tx_type {
            super::transaction_type::TransactionType::deposit => {
                self.transactions.insert(tx_id, tx);
                client.respond_to_deposit(tx_amt)?;
                Ok(())
            }
            super::transaction_type::TransactionType::withdrawal => {
                self.transactions.insert(tx_id, tx);
                client.respond_to_withdraw(tx_amt)?;
                Ok(())
            }
            super::transaction_type::TransactionType::dispute => {
                let mut inner = opt_tx?;
                inner.mark_under_dispute()?;
                client.respond_to_dispute(inner.get_amt())?;
                self.transactions.insert(inner.get_id(), inner);
                Ok(())
            }
            super::transaction_type::TransactionType::resolve => {
                let mut inner = opt_tx?;
                inner.mark_resolved()?;
                client.respond_to_resolve(inner.get_amt())?;
                self.transactions.insert(inner.get_id(), inner);
                Ok(())
            }
            super::transaction_type::TransactionType::chargeback => {
                let mut inner = opt_tx?;
                inner.mark_chargebacked()?;
                client.respond_to_chargeback(inner.get_amt())?;
                self.transactions.insert(inner.get_id(), inner);
                Ok(())
            }
        }
    }

    fn find_transaction(&self, transaction_id: u32) -> Result<Transaction, Box<dyn Error>> {
        let tx = self
            .transactions
            .get(&transaction_id)
            .ok_or(TransactionError::TransactionNotFound(transaction_id))?;
        Ok(tx.clone())
    }
}

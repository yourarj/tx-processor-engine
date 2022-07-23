use serde::{Deserialize, Serialize};

use super::{
    error::TransactionError, transaction_status::TransactionStatus,
    transaction_type::TransactionType,
};

/// Transaction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// type of transaction
    #[serde(rename = "type")]
    tx_type: TransactionType,
    /// client ID
    client: u16,
    /// transaction ID
    tx: u32,
    /// amount being involved in transaction.
    amount: Option<f64>,
    /// state of transaction
    /// under_dispute / resolved / chargebacked
    #[serde(skip)]
    status: Option<TransactionStatus>,
}

impl Transaction {
    /// get transaction id
    pub fn get_id(&self) -> u32 {
        self.tx
    }

    /// get client id associated with current transaction
    pub fn get_client_id(&self) -> u16 {
        self.client
    }

    pub fn get_amt(&self) -> f64 {
        self.amount.unwrap_or(0.0_f64)
    }

    pub fn get_type(&self) -> TransactionType {
        self.tx_type.clone()
    }

    /// check the conditions and mark a transactions as
    /// under dispute
    pub fn mark_under_dispute(&mut self) -> Result<(), TransactionError> {
        match self.status {
            None => {
                self.status = Some(TransactionStatus::under_dispute);
                Ok(())
            }
            _ => Err(TransactionError::InvalidTransactionState(self.get_id())),
        }
    }

    /// check the conditions and mark a transactions as
    /// chargebacked
    pub fn mark_resolved(&mut self) -> Result<(), TransactionError> {
        match self.status {
            Some(TransactionStatus::under_dispute) => {
                self.status = Some(TransactionStatus::resolved);
                Ok(())
            }
            _ => Err(TransactionError::InvalidTransactionState(self.get_id())),
        }
    }

    /// check the conditions and mark a transactions as
    /// chargebacked
    pub fn mark_chargebacked(&mut self) -> Result<(), TransactionError> {
        match self.status {
            Some(TransactionStatus::under_dispute) => {
                self.status = Some(TransactionStatus::chargebacked);
                Ok(())
            }
            _ => Err(TransactionError::InvalidTransactionState(self.get_id())),
        }
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Can't act on finalized/reverted transaction, tx_id `{0}`")]
    InvalidTransactionState(u32),

    #[error("Unable to find specified transaction, tx_id: `{0}`")]
    TransactionNotFound(u32),  

}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Can't act on finalized/reverted transaction, tx_id `{0}`")]
    InvalidTransactionState(u32),

    #[error("Unable to find specified transaction, tx_id: `{0}`")]
    TransactionNotFound(u32),

    #[error("Invalid transaction amount `{1}` found, tx_id: `{0}`")]
    InvalidTransactionAmount(u32, f64),
}

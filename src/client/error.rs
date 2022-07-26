use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Insufficient Account Balance in account `#ID: {0}`")]
    InsufficientAccountBalance(u16),

    #[error("Client account `#ID: {0}` is locked")]
    ClientAccountLocked(u16),

    #[error("Client account `#ID: {0}` funds would overflow")]
    ClientFundsWouldOverflow(u16),
}

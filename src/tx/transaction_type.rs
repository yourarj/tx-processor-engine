use serde::{Deserialize, Serialize};

/// Transaction Type
#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TransactionType {
    /// deposit of fiat
    deposit,

    /// withdrawal of fiat
    withdrawal,

    /// raise a dispute against a transaction
    dispute,

    /// mark a dispute as resolved
    resolve,

    /// mark a dispute as chargeback
    chargeback,
}

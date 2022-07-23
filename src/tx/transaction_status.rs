use serde::{Deserialize, Serialize};

/// Transaction Type
#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TransactionStatus {
    /// under dispute
    under_dispute,

    /// resolved
    resolved,

    /// chargebacked
    chargebacked,
}

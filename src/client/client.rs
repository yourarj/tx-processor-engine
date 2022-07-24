use serde::{Deserialize, Serialize};

// TODO add missing documentation
use super::error::ClientError;

// client account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    /// client ID
    client: u16,
    /// available amount
    #[serde(serialize_with = "round_serialize_f64")]
    available: f64,
    /// held amount
    #[serde(serialize_with = "round_serialize_f64")]
    held: f64,
    /// total amount
    #[serde(serialize_with = "round_serialize_f64")]
    total: f64,
    /// is this client account is locked/ freezed
    locked: bool,
}
// way to construct new client
impl Client {
    pub fn new(client_id: u16) -> Self {
        Self {
            client: client_id,
            available: Default::default(),
            held: Default::default(),
            total: Default::default(),
            locked: Default::default(),
        }
    }

    pub fn get_client(&self) -> u16 {
        self.client
    }

    pub fn get_available(&self) -> f64 {
        self.available
    }

    pub fn get_held(&self) -> f64 {
        self.held
    }

    pub fn get_total(&self) -> f64 {
        self.total
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }
}

impl Client {
    /// actions to take as response to deposit is requested
    pub fn respond_to_deposit(&mut self, amt: f64) -> Result<(), ClientError> {
        // check if account is not locked
        if self.locked {
            Err(ClientError::ClientAccountLocked(self.client))
        } else {
            self.available += amt;
            self.total += amt;
            Ok(())
        }
    }

    /// actions to take as response to withdraw is requested
    pub fn respond_to_withdraw(&mut self, amt: f64) -> Result<(), ClientError> {
        if self.available < amt || self.total < amt {
            Err(ClientError::InsufficientAccountBalance(self.client))
        } else if self.locked {
            Err(ClientError::ClientAccountLocked(self.client))
        } else {
            self.available -= amt;
            self.total -= amt;
            Ok(())
        }
    }

    // actions to take when dispute is requested
    pub fn respond_to_dispute(&mut self, amt: f64) -> Result<(), ClientError> {
        if self.available < amt || self.total < amt {
            Err(ClientError::InsufficientAccountBalance(self.client))
        } else if self.locked {
            Err(ClientError::ClientAccountLocked(self.client))
        } else {
            // decrease available balance
            self.available -= amt;

            // increase available balance
            self.held += amt;
            Ok(())
        }
    }

    // actions to take resolve is requested
    pub fn respond_to_resolve(&mut self, amt: f64) -> Result<(), ClientError> {
        if self.held < amt {
            Err(ClientError::InsufficientAccountBalance(self.client))
        } else if self.locked {
            Err(ClientError::ClientAccountLocked(self.client))
        } else {
            // decrease held balance
            self.held -= amt;

            // increase available balance
            self.available += amt;
            Ok(())
        }
    }

    // actions to perform when chargeback is requested
    pub fn respond_to_chargeback(&mut self, amt: f64) -> Result<(), ClientError> {
        if self.held < amt || self.total < amt {
            Err(ClientError::InsufficientAccountBalance(self.client))
        } else if self.locked {
            Err(ClientError::ClientAccountLocked(self.client))
        } else {
            // decrement held
            self.held -= amt;
            // decrement total
            self.total -= amt;
            // lock the client account
            self.locked = true;
            Ok(())
        }
    }
}

use serde::Serializer;

fn round_serialize_f64<S>(x: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_f64(format!("{:.4}", x).parse().unwrap())
}

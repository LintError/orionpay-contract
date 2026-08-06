//! Shared data types used across all OrionPay contracts

use soroban_sdk::{Address, String, contracttype};

#[contracttype]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment {
    /// Unique identifier for the payment
    pub id: String,
    /// Sender of the payment
    pub sender: Address,
    /// Recipient of the payment
    pub recipient: Address,
    /// Amount of the payment in smallest units
    pub amount: i128,
    /// Asset code (e.g., "USDC", "XLM")
    pub asset: String,
    /// Timestamp when the payment was created
    pub created_at: u64,
    /// Timestamp when the payment was completed (if applicable)
    pub completed_at: Option<u64>,
}

#[contracttype]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipient {
    /// Address of the recipient
    pub address: Address,
    /// Percentage of the total amount this recipient should receive
    pub percentage: u32,
}

#[contracttype]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The operation is pending
    Pending = 0,
    /// The operation is processing (intermediate state)
    Processing = 1,
    /// The operation completed successfully
    Completed = 2,
    /// The operation failed
    Failed = 3,
    /// The operation was cancelled
    Cancelled = 4,
}

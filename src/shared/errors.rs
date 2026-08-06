//! Shared error types used across all OrionPay contracts

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum OrionError {
    /// Generic unauthorized error
    Unauthorized = 1,
    /// Insufficient balance to perform the operation
    InsufficientBalance = 2,
    /// Payment amount is invalid (zero or negative)
    InvalidAmount = 3,
    /// Recipient address is invalid
    InvalidRecipient = 4,
    /// Contract has already been initialized
    AlreadyInitialized = 5,
    /// Contract has not been initialized yet
    NotInitialized = 6,
    /// The requested resource was not found
    NotFound = 7,
    /// The operation is not supported in the current state
    InvalidState = 8,
    /// Deadline has passed for the operation
    DeadlineExceeded = 9,
    /// The signature provided is invalid
    InvalidSignature = 10,
    /// Percentage values don't add up to 100%
    InvalidPercentage = 11,
}

impl OrionError {
    /// Convert the error to a u32 for use in Soroban's error handling
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }
}

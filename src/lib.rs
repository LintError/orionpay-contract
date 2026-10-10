#![no_std]
extern crate alloc;
use soroban_sdk::{Env, String, contract, contractimpl};

// Shared modules that contain common types and errors used across all features
pub mod shared {
    pub use crate::shared::errors::OrionError;
    pub use crate::shared::types::{Payment, Recipient, Status};

    mod errors;
    mod types;

    /// Collision-free record ID generation shared by every module.
    pub mod ids;
}

// Core feature modules
pub mod bulk_payments;
pub mod payments;
pub mod split_payments;

#[contract]
pub struct OrionPay;

#[contractimpl]
impl OrionPay {
    /// Version of the OrionPay contract
    pub fn version(env: Env) -> String {
        String::from_str(&env, "0.1.0")
    }

    /// Simple hello function that greets the given name: "Hello <to>"
    pub fn hello(env: Env, to: String) -> String {
        let len = to.len() as usize;
        let mut name_buf = alloc::vec![0u8; len];
        to.copy_into_slice(&mut name_buf);

        let mut greeting = alloc::vec::Vec::from(&b"Hello "[..]);
        greeting.extend_from_slice(&name_buf);

        let greeting_str = core::str::from_utf8(&greeting).unwrap_or("Hello");
        String::from_str(&env, greeting_str)
    }
}

#[cfg(test)]
mod test;

#![no_std]
extern crate alloc;

// Core feature modules
pub mod bulk_payments;
pub mod payments;
pub mod split_payments;

// Shared modules that contain common types and errors used across all features
pub mod shared {
    pub use crate::shared::errors::OrionError;
    pub use crate::shared::types::{Payment, Recipient, Status};

    mod errors;
    mod types;
}

// The scaffold contract entry points below are for local testing only.
//
// `orionpay-stellar` is the deployable contract and wraps these modules. If this
// crate's `#[contractimpl]` were compiled into the same wasm module as the
// wrapper, the two would each export a `version` symbol and the link would fail
// with `duplicate symbol: version`. Gating on `scaffold-contract` (off by
// default) keeps the wasm build to one contract surface; tests enable it via
// `--features testutils`.
//
// This stays at the crate root rather than in a submodule because
// `#[contractimpl]` generates the `OrionPayClient` type alongside the impl, and
// `src/test.rs` refers to it unqualified.
#[cfg(feature = "scaffold-contract")]
use soroban_sdk::{Env, String, contract, contractimpl};

#[cfg(feature = "scaffold-contract")]
#[contract]
pub struct OrionPay;

#[cfg(feature = "scaffold-contract")]
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

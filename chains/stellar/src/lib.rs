#![no_std]
extern crate alloc;

use orionpay_contract::bulk_payments::{self, BulkPayment, BulkPaymentRecipient};
use orionpay_contract::payments::Payments;
use orionpay_contract::shared::{OrionError, Payment as SharedPayment};
use orionpay_contract::split_payments::{self, SplitPayment, SplitRecipient};
use soroban_sdk::{Address, Env, String, Vec, contract, contractimpl};

#[contract]
pub struct OrionPayStellar;

#[contractimpl]
impl OrionPayStellar {
    /// Get the contract version
    pub fn version(env: Env) -> String {
        String::from_str(&env, "0.1.0-stellar")
    }

    // ============ Single Payments ============

    /// Create a new single payment
    pub fn create_payment(
        env: Env,
        sender: Address,
        recipient: Address,
        amount: i128,
        asset: String,
        memo: Option<String>,
    ) -> Result<SharedPayment, OrionError> {
        Payments::create(env, sender, recipient, amount, asset, memo)
    }

    /// Complete a payment by ID
    pub fn complete_payment(env: Env, payment_id: String) -> Result<(), OrionError> {
        Payments::complete(env, payment_id)
    }

    /// Get a payment by ID
    pub fn get_payment(env: Env, payment_id: String) -> Result<SharedPayment, OrionError> {
        Payments::get_payment(env, payment_id)
    }

    // ============ Split Payments ============

    /// Create a new split payment
    pub fn create_split_payment(
        env: Env,
        sender: String,
        total_amount: u64,
        recipients: Vec<SplitRecipient>,
    ) -> Result<String, OrionError> {
        split_payments::create_split_payment(env, sender, total_amount, recipients)
    }

    /// Process a recipient in a split payment
    pub fn process_split_recipient(
        env: Env,
        split_id: String,
        recipient_index: u32,
    ) -> Result<(), OrionError> {
        split_payments::process_recipient(env, split_id, recipient_index)
    }

    /// Get a split payment by ID
    pub fn get_split_payment(env: Env, split_id: String) -> Result<SplitPayment, OrionError> {
        split_payments::get_split_payment(env, split_id)
    }

    // ============ Bulk Payments ============

    /// Create a new bulk payment
    pub fn create_bulk_payment(
        env: Env,
        sender: String,
        recipients: Vec<BulkPaymentRecipient>,
    ) -> Result<String, OrionError> {
        bulk_payments::create_bulk_payment(env, sender, recipients)
    }

    /// Process a recipient in a bulk payment
    pub fn process_bulk_recipient(
        env: Env,
        bulk_id: String,
        recipient_index: u32,
    ) -> Result<(), OrionError> {
        bulk_payments::process_recipient(env, bulk_id, recipient_index)
    }

    /// Get a bulk payment by ID
    pub fn get_bulk_payment(env: Env, bulk_id: String) -> Result<BulkPayment, OrionError> {
        bulk_payments::get_bulk_payment(env, bulk_id)
    }
}

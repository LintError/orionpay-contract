use crate::shared::{OrionError, Payment as SharedPayment, Status, ids};
use soroban_sdk::{Address, Env, String};

// Extended payment status for payments module
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExtendedPaymentStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
}

impl From<ExtendedPaymentStatus> for Status {
    fn from(status: ExtendedPaymentStatus) -> Self {
        match status {
            ExtendedPaymentStatus::Pending => Status::Pending,
            ExtendedPaymentStatus::Completed => Status::Completed,
            ExtendedPaymentStatus::Failed => Status::Failed,
            ExtendedPaymentStatus::Refunded => Status::Cancelled,
        }
    }
}

/// Payments module handles single direct payments between accounts
pub struct Payments;

impl Payments {
    /// Create a new payment
    pub fn create(
        env: Env,
        sender: Address,
        recipient: Address,
        amount: i128,
        asset: String,
        _memo: Option<String>,
    ) -> Result<SharedPayment, OrionError> {
        sender.require_auth();

        if amount <= 0 {
            return Err(OrionError::InvalidAmount);
        }

        let payment_id = Self::generate_payment_id(&env);

        let payment = SharedPayment {
            id: payment_id.clone(),
            sender,
            recipient,
            amount,
            asset,
            created_at: env.ledger().timestamp(),
            completed_at: None,
        };

        // Store payment in ledger
        env.storage().persistent().set(&payment_id, &payment);

        Ok(payment)
    }

    /// Complete a pending payment
    pub fn complete(env: Env, payment_id: String) -> Result<(), OrionError> {
        let mut payment: SharedPayment = env
            .storage()
            .persistent()
            .get(&payment_id)
            .ok_or(OrionError::NotFound)?;

        if payment.completed_at.is_some() {
            return Err(OrionError::InvalidState);
        }

        payment.completed_at = Some(env.ledger().timestamp());
        env.storage().persistent().set(&payment_id, &payment);

        Ok(())
    }

    /// Generate a unique payment ID
    ///
    /// IDs come from the shared helper so they stay unique even when two
    /// payments are created in the same ledger. Previously the ID was derived
    /// from the timestamp alone, so the second create in a ledger overwrote the
    /// first payment's record.
    fn generate_payment_id(env: &Env) -> String {
        ids::generate(env, "payment")
    }

    /// Get payment details by ID
    pub fn get_payment(env: Env, payment_id: String) -> Result<SharedPayment, OrionError> {
        env.storage()
            .persistent()
            .get(&payment_id)
            .ok_or(OrionError::NotFound)
    }
}

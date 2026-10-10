use crate::shared::{OrionError, Status, ids};
use soroban_sdk::{Env, String, contracttype};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BulkPaymentRecipient {
    pub address: String,
    pub amount: u64,
    pub completed: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BulkPayment {
    pub id: String,
    pub sender: String,
    pub recipients: soroban_sdk::Vec<BulkPaymentRecipient>,
    pub completed_count: u32,
    pub status: Status,
    pub created_at: u64,
}

/// Namespace prefix for bulk-payment record IDs.
const BULK_NAMESPACE: &str = "bulk";

/// Generate a unique bulk-payment ID.
///
/// Namespaced via [`ids::generate`] so bulk IDs cannot collide with split IDs,
/// which previously shared the bare-timestamp keyspace.
fn generate_bulk_id(env: &Env) -> String {
    ids::generate(env, BULK_NAMESPACE)
}

pub fn create_bulk_payment(
    env: Env,
    sender: String,
    recipients: soroban_sdk::Vec<BulkPaymentRecipient>,
) -> Result<String, OrionError> {
    let id = generate_bulk_id(&env);

    let bulk = BulkPayment {
        id: id.clone(),
        sender,
        recipients: recipients.clone(),
        completed_count: 0,
        status: Status::Pending,
        created_at: env.ledger().timestamp(),
    };

    env.storage().persistent().set(&id, &bulk);
    Ok(id)
}

pub fn process_recipient(
    env: Env,
    bulk_id: String,
    recipient_index: u32,
) -> Result<(), OrionError> {
    let mut bulk: BulkPayment = env
        .storage()
        .persistent()
        .get(&bulk_id)
        .ok_or(OrionError::NotFound)?;

    if bulk.status == Status::Completed {
        return Ok(());
    }

    if recipient_index >= bulk.recipients.len() {
        return Err(OrionError::InvalidRecipient);
    }

    let mut recipient = bulk.recipients.get(recipient_index).unwrap();
    if !recipient.completed {
        recipient.completed = true;
        bulk.recipients.set(recipient_index, recipient);
        bulk.completed_count += 1;

        // Update status based on completion
        if bulk.completed_count == bulk.recipients.len() {
            bulk.status = Status::Completed;
        } else {
            bulk.status = Status::Processing;
        }

        env.storage().persistent().set(&bulk_id, &bulk);
    }

    Ok(())
}

/// Get a bulk payment by ID.
///
/// # Errors
///
/// * [`OrionError::NotFound`] - no bulk payment with `bulk_id`, or `bulk_id`
///   belongs to another record namespace (for example a payment or split ID)
pub fn get_bulk_payment(env: Env, bulk_id: String) -> Result<BulkPayment, OrionError> {
    // Reject IDs from other namespaces before touching storage: the ID is the
    // storage key, so a foreign ID could otherwise resolve to a real record of
    // a different type and fail to decode.
    if !ids::is_in_namespace(&env, &bulk_id, BULK_NAMESPACE) {
        return Err(OrionError::NotFound);
    }

    env.storage()
        .persistent()
        .get(&bulk_id)
        .ok_or(OrionError::NotFound)
}

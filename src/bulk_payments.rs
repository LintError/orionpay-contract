use crate::shared::{OrionError, Status};
use alloc::string::ToString;
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

fn generate_bulk_id(env: &Env) -> String {
    // Create unique ID using timestamp which is already unique
    let timestamp = env.ledger().timestamp();
    String::from_str(env, &timestamp.to_string())
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

pub fn get_bulk_payment(env: Env, bulk_id: String) -> Result<BulkPayment, OrionError> {
    env.storage()
        .persistent()
        .get(&bulk_id)
        .ok_or(OrionError::NotFound)
}

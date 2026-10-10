use crate::shared::{OrionError, Status, ids};
use soroban_sdk::{Env, String, contracttype};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitRecipient {
    pub address: String,
    pub percentage: u32, // Percentage as integer (e.g., 50 for 50%)
    pub amount: u64,
    pub completed: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitPayment {
    pub id: String,
    pub sender: String,
    pub total_amount: u64,
    pub recipients: soroban_sdk::Vec<SplitRecipient>,
    pub completed_count: u32,
    pub status: Status,
    pub created_at: u64,
}

/// Generate a unique split-payment ID.
///
/// Namespaced via [`ids::generate`] so split IDs cannot collide with bulk IDs,
/// which previously shared the bare-timestamp keyspace.
fn generate_split_id(env: &Env) -> String {
    ids::generate(env, "split")
}

pub fn create_split_payment(
    env: Env,
    sender: String,
    total_amount: u64,
    mut recipients: soroban_sdk::Vec<SplitRecipient>,
) -> Result<String, OrionError> {
    // Verify percentages add up to 100%
    let mut total_percentage: u32 = 0;
    for recipient in recipients.iter() {
        total_percentage += recipient.percentage;
        // Calculate the amount for each recipient based on percentage
        let calculated_amount = (total_amount as u128 * recipient.percentage as u128 / 100) as u64;
        let mut r = recipient.clone();
        r.amount = calculated_amount;
        // Update the recipient in the vector
        let idx = recipients.first_index_of(&recipient).unwrap();
        recipients.set(idx, r);
    }

    if total_percentage != 100 {
        return Err(OrionError::InvalidPercentage);
    }

    let id = generate_split_id(&env);

    let split = SplitPayment {
        id: id.clone(),
        sender,
        total_amount,
        recipients,
        completed_count: 0,
        status: Status::Pending,
        created_at: env.ledger().timestamp(),
    };

    env.storage().persistent().set(&id, &split);
    Ok(id)
}

pub fn process_recipient(
    env: Env,
    split_id: String,
    recipient_index: u32,
) -> Result<(), OrionError> {
    let mut split: SplitPayment = env
        .storage()
        .persistent()
        .get(&split_id)
        .ok_or(OrionError::NotFound)?;

    if split.status == Status::Completed {
        return Ok(());
    }

    if recipient_index >= split.recipients.len() {
        return Err(OrionError::InvalidRecipient);
    }

    let mut recipient = split.recipients.get(recipient_index).unwrap();
    if !recipient.completed {
        recipient.completed = true;
        split.recipients.set(recipient_index, recipient);
        split.completed_count += 1;

        // Update status based on completion
        if split.completed_count == split.recipients.len() {
            split.status = Status::Completed;
        } else {
            split.status = Status::Processing;
        }

        env.storage().persistent().set(&split_id, &split);
    }

    Ok(())
}

pub fn get_split_payment(env: Env, split_id: String) -> Result<SplitPayment, OrionError> {
    env.storage()
        .persistent()
        .get(&split_id)
        .ok_or(OrionError::NotFound)
}

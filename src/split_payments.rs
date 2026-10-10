use crate::shared::{OrionError, Status, ids};
use soroban_sdk::{Address, Env, String, contracttype};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitRecipient {
    /// Destination of this recipient's payout leg.
    ///
    /// Typed as `Address` (rather than `String`) so `process_recipient` can
    /// require the recipient's own authorization.
    pub address: Address,
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

/// Namespace prefix for split-payment record IDs.
const SPLIT_NAMESPACE: &str = "split";

/// Generate a unique split-payment ID.
///
/// Namespaced via [`ids::generate`] so split IDs cannot collide with bulk IDs,
/// which previously shared the bare-timestamp keyspace.
fn generate_split_id(env: &Env) -> String {
    ids::generate(env, SPLIT_NAMESPACE)
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

/// Mark the recipient at `recipient_index` as completed.
///
/// Only the recipient named at that index may mark itself complete: `authorizer`
/// must sign the call and must equal the stored recipient address. Marking a
/// recipient complete is what authorizes that payout leg, so an unauthorized
/// caller must not be able to advance the split on someone else's behalf.
///
/// # Errors
///
/// * [`OrionError::NotFound`] - no split payment with `split_id`
/// * [`OrionError::InvalidRecipient`] - `recipient_index` is out of range
/// * [`OrionError::Unauthorized`] - `authorizer` is not the recipient at
///   `recipient_index`
pub fn process_recipient(
    env: Env,
    split_id: String,
    recipient_index: u32,
    authorizer: Address,
) -> Result<(), OrionError> {
    // Require the caller to sign before any state is read or written.
    authorizer.require_auth();

    if !ids::is_in_namespace(&env, &split_id, SPLIT_NAMESPACE) {
        return Err(OrionError::NotFound);
    }

    let mut split: SplitPayment = env
        .storage()
        .persistent()
        .get(&split_id)
        .ok_or(OrionError::NotFound)?;

    // Bounds check before the `Completed` early return, so an out-of-range
    // index is reported as `InvalidRecipient` rather than silently succeeding.
    if recipient_index >= split.recipients.len() {
        return Err(OrionError::InvalidRecipient);
    }

    let mut recipient = split.recipients.get(recipient_index).unwrap();

    // The signer must be the recipient being completed: marking a recipient
    // complete is what authorizes that payout leg.
    if recipient.address != authorizer {
        return Err(OrionError::Unauthorized);
    }

    if split.status == Status::Completed {
        return Ok(());
    }

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

/// Get a split payment by ID.
///
/// # Errors
///
/// * [`OrionError::NotFound`] - no split payment with `split_id`, or `split_id`
///   belongs to another record namespace (for example a payment or bulk ID)
pub fn get_split_payment(env: Env, split_id: String) -> Result<SplitPayment, OrionError> {
    // Reject IDs from other namespaces before touching storage: the ID is the
    // storage key, so a foreign ID could otherwise resolve to a real record of
    // a different type and fail to decode.
    if !ids::is_in_namespace(&env, &split_id, SPLIT_NAMESPACE) {
        return Err(OrionError::NotFound);
    }

    env.storage()
        .persistent()
        .get(&split_id)
        .ok_or(OrionError::NotFound)
}

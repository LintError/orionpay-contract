//! Shared record identifier generation.
//!
//! Every module that persists a record must derive its identifier here so
//! that identifiers stay unique across modules and within a single ledger.
//!
//! Historically each module built its own ID from `env.ledger().timestamp()`
//! alone. That made two classes of collision possible:
//!
//! * Two records created in the same ledger shared an ID, so the second
//!   `set()` silently overwrote the first.
//! * `split` and `bulk` IDs were both the bare timestamp, with no namespace,
//!   so a split payment was readable through the bulk-payment accessor.

use soroban_sdk::{Env, String, contracttype};

/// Storage keys owned by this module.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Monotonically increasing record counter shared by every namespace.
    RecordSeq,
}

/// Generate a collision-free record ID for `namespace`.
///
/// IDs have the form `{namespace}_{seq}` (for example `payment_1`,
/// `split_2`). The counter is stored in instance storage, so two records can
/// never share an ID even when created in the same ledger, and the namespace
/// prefix keeps record types from addressing each other's storage entries.
///
/// # Examples
///
/// ```
/// use soroban_sdk::Env;
/// use orionpay_contract::shared::ids;
///
/// let env = Env::default();
/// let a = ids::generate(&env, "split");
/// let b = ids::generate(&env, "bulk");
/// assert_ne!(a, b);
/// ```
pub fn generate(env: &Env, namespace: &str) -> String {
    let seq = next_seq(env);
    // `soroban_sdk::String` has no `push_str`, so format once and convert.
    String::from_str(env, &alloc::format!("{}_{}", namespace, seq))
}

/// Increment and return the shared record counter.
fn next_seq(env: &Env) -> u64 {
    let instance = env.storage().instance();
    let current: u64 = instance.get(&DataKey::RecordSeq).unwrap_or(0);
    // `saturating_add` keeps the contract total if the counter ever maxes out;
    // at that point reuse is preferable to an arithmetic panic.
    let next = current.saturating_add(1);
    instance.set(&DataKey::RecordSeq, &next);
    next
}

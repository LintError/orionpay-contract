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
/// # Example
///
/// `generate` writes to instance storage, so it must be called from within a
/// contract context; see the `test_split_and_bulk_ids_are_namespaced` test in
/// `src/test.rs` for a runnable example.
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

/// Whether `id` was generated for `namespace`.
///
/// Record IDs are used directly as storage keys, so a getter that is handed an
/// ID from another namespace would otherwise find a real record and try to
/// decode it as the wrong type, aborting with a host error instead of returning
/// [`crate::shared::OrionError::NotFound`]. Getters use this to reject
/// foreign-namespace IDs up front.
///
/// # Examples
///
/// ```
/// use soroban_sdk::{Env, String};
/// use orionpay_contract::shared::ids;
///
/// let env = Env::default();
/// // `is_in_namespace` reads no storage, so it works outside a contract.
/// let split_id = String::from_str(&env, "split_7");
/// assert!(ids::is_in_namespace(&env, &split_id, "split"));
/// assert!(!ids::is_in_namespace(&env, &split_id, "bulk"));
/// assert!(!ids::is_in_namespace(&env, &String::from_str(&env, "split"), "split"));
/// ```
pub fn is_in_namespace(env: &Env, id: &String, namespace: &str) -> bool {
    // Compare the namespace prefix byte-for-byte. `copy_into_slice` requires an
    // exactly-sized destination, so read the whole ID and slice the result.
    let prefix = String::from_str(env, namespace);
    let prefix_len = prefix.len() as usize;
    let id_len = id.len() as usize;

    // `generate` writes a `_` separator after the namespace, so a valid ID is
    // strictly longer than the namespace itself.
    if id_len <= prefix_len {
        return false;
    }

    let mut id_bytes = alloc::vec![0u8; id_len];
    id.copy_into_slice(&mut id_bytes);
    let mut prefix_bytes = alloc::vec![0u8; prefix_len];
    prefix.copy_into_slice(&mut prefix_bytes);

    id_bytes[..prefix_len] == prefix_bytes[..] && id_bytes[prefix_len] == b'_'
}

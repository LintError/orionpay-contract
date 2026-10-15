use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_version() {
    let env = Env::default();
    let contract_id = env.register(OrionPayStellar, ());
    let client = OrionPayStellarClient::new(&env, &contract_id);

    // The reported version must match the manifest plus the chain qualifier
    // (`-stellar`), so a version bump in `Cargo.toml` is picked up automatically.
    let version = client.version();
    assert_eq!(
        version,
        String::from_str(&env, concat!(env!("CARGO_PKG_VERSION"), "-stellar"))
    );
}

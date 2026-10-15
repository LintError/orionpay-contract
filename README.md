# 🦀 OrionPay Contracts

Rust smart contracts for OrionPay, targeting the Stellar (Soroban) blockchain with a multi-chain structure.

[![CI](https://github.com/YOUR_ORG/orionpay-contract/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_ORG/orionpay-contract/actions/workflows/ci.yml)

## Features

Implemented:

- **Payments** — single direct payments (create, complete, get)
- **Split Payments** — distribute a total across recipients by percentage
- **Bulk Payments** — pay many recipients in one operation

Planned (not yet implemented):

- Escrow
- Subscriptions

Voice commands are supported via the backend (out of scope for these contracts).

---

## Chains

- **Stellar** — implemented (`chains/stellar`)
- Ethereum, Polygon, Solana — placeholders for future work

---

## Structure

```
.
├── src/                  # Core contract logic (orionpay-contract crate)
│   ├── payments.rs
│   ├── split_payments.rs
│   ├── bulk_payments.rs
│   └── shared/           # Shared types (Payment, Recipient, Status) and OrionError
├── chains/
│   └── stellar/          # Stellar-specific contract wrapper (orionpay-stellar crate)
├── contracts/            # (reserved)
└── Cargo.toml            # Cargo workspace root
```

The repository is a Cargo **workspace** with two members: `orionpay-contract` (core) and
`orionpay-stellar` (the Stellar contract that wraps the core modules).

---

## Build & Test

**Requirements:** Rust 1.93+ (MSRV), installed via [rustup](https://rustup.rs/). A
`rust-toolchain.toml` at the repository root pins the exact toolchain, so `rustup`
selects it on a fresh clone and a plain `cargo build` reproduces CI. Both crates also
declare `rust-version = "1.93"` in their `Cargo.toml`.

```bash
# Build every crate in the workspace
cargo build --workspace

# Run the full test suite (test-only helpers live behind the `testutils` feature)
cargo test --workspace --features testutils

# Lint (matches CI)
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --features testutils -- -D warnings

# Build the deployable Wasm contracts
cargo build --workspace --target wasm32v1-none --release
```

The Wasm target is `wasm32v1-none`, not `wasm32-unknown-unknown`: the Soroban
environment does not support the `reference-types` and `multi-value` features
that Rust 1.82+ enables for `wasm32-unknown-unknown`. Install the target with
`rustup target add wasm32v1-none`. Building for `wasm32-unknown-unknown` fails
in `soroban-sdk`'s build script.

The deployable artifact is `chains/stellar` (`orionpay_stellar.wasm`); the core
`orionpay-contract` crate is a library that it wraps.

CI runs all of the above on every push and pull request — see
[.github/workflows/ci.yml](.github/workflows/ci.yml).

---

## Contributing

We welcome contributions! Please see:

- **[CONTRIBUTING.md](CONTRIBUTING.md)** — how to build, test, and submit PRs
- **[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)** — our community standards
- **[SECURITY.md](SECURITY.md)** — how to report security vulnerabilities

For bug reports and feature requests, use our [issue templates](.github/ISSUE_TEMPLATE/).

---

## License

MIT — see [LICENSE](LICENSE).


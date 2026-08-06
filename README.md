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

```bash
# Build every crate in the workspace
cargo build --workspace

# Run the full test suite
cargo test --workspace

# Lint (matches CI)
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Build the deployable Wasm contracts
cargo build --workspace --target wasm32-unknown-unknown --release
```

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


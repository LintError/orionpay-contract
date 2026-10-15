# Contributing to OrionPay Contracts

Thank you for your interest in contributing to OrionPay! This guide will help you get started and ensure your contributions can be reviewed and merged smoothly.

## Getting Started

### Prerequisites

- **Rust** — install via [rustup](https://rustup.rs/). The pinned MSRV is
  **Rust 1.93**; `rust-toolchain.toml` at the repository root makes `rustup` select it
  automatically on a fresh clone (so a plain `cargo build` reproduces CI). This is a
  floor, not a ceiling: any newer stable works.
- **wasm32v1-none target** — `rustup target add wasm32v1-none`
- **Git** for version control

> Use `wasm32v1-none`, **not** `wasm32-unknown-unknown`. The Soroban environment
> rejects the latter on Rust 1.82+ because that target enables `reference-types`
> and `multi-value`, and the build fails inside `soroban-sdk`'s build script.

### Building the Project

This is a Cargo workspace with two crates. Build both:

```bash
cargo build --workspace
```

### Running Tests

Always run the full test suite before submitting:

```bash
cargo test --workspace --features testutils
```

The `testutils` feature is required: it enables the Soroban test helpers the
suite uses, and it is deliberately off by default so those host-only helpers
never reach the Wasm build.

All tests must pass. Our CI enforces this.

## Making Changes

### 1. Fork and Clone

Fork the repository, then clone your fork:

```bash
git clone https://github.com/YOUR_USERNAME/orionpay-contract.git
cd orionpay-contract
```

### 2. Create a Branch

Use a descriptive branch name:

```bash
git checkout -b fix/payment-validation
# or
git checkout -b feature/add-escrow
```

### 3. Code Standards

Before committing, ensure your code passes all checks (these run in CI):

```bash
# Format check
cargo fmt --all -- --check

# Auto-format if needed
cargo fmt --all

# Lint with zero warnings
cargo clippy --workspace --all-targets --features testutils -- -D warnings

# Tests
cargo test --workspace --features testutils

# Build the deployable Wasm contracts
cargo build --workspace --target wasm32v1-none --release
```

**CI will fail if any of these fail**, so run them locally first.

### 4. Commit Guidelines

- Write clear commit messages describing *what* and *why*
- Keep commits focused on a single logical change
- Reference issue numbers when applicable: `Fix #42: validate split payment percentages`

### 5. Testing Your Changes

- Add tests for new features
- Add tests for bug fixes to prevent regressions
- Ensure tests cover both success and error paths
- Tests must use `env.as_contract()` for storage operations (see `src/test.rs` for examples)

## Submitting a Pull Request

1. **Push your branch** to your fork
2. **Open a Pull Request** against the `main` branch
3. **Fill out the PR template** (describes what changed and why)
4. **Wait for CI** — all checks must pass (format, clippy, build, tests)
5. **Respond to review feedback** promptly

### PR Review Process

- Maintainers typically respond within 48 hours
- CI must be green before merge
- At least one maintainer approval is required
- Keep PRs focused — large changes may be asked to split

## Development Tips

### Project Structure

```
src/              # Core contract logic (orionpay-contract crate)
  payments.rs
  split_payments.rs
  bulk_payments.rs
  shared/         # Shared types and errors
chains/stellar/   # Stellar-specific wrapper (orionpay-stellar crate)
```

### Common Issues

**Tests failing with "not accessible outside of a contract"?**
→ Wrap storage operations in `env.as_contract(&contract_id, || { ... })`

**Clippy warnings about unused variables?**
→ Remove them or prefix with `_` if they're required by a trait

**Format check failing?**
→ Run `cargo fmt --all` before committing

### Writing Tests

Use the Soroban SDK test utilities:

```rust
#[test]
fn test_something() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    env.mock_all_auths();
    
    // Storage ops need contract context
    let result = env.as_contract(&contract_id, || {
        Payments::create(env.clone(), sender, recipient, 100, asset, None)
    }).unwrap();
    
    assert_eq!(result.amount, 100);
}
```

## Security

This is a **payment contract** handling financial transactions. Security is critical:

- Never ignore error cases
- Validate all inputs (amounts, addresses, percentages)
- Test edge cases (zero amounts, overflow, invalid state transitions)
- If you discover a security vulnerability, see [SECURITY.md](SECURITY.md) for responsible disclosure

## Code of Conduct

Be respectful, constructive, and welcoming. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

## Questions?

- Check existing [issues](../../issues) and [pull requests](../../pulls)
- Open a new issue for bugs or feature requests
- Tag issues appropriately: `bug`, `enhancement`, `question`, `good first issue`

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

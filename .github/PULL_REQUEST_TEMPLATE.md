<!--
Thanks for contributing to OrionPay! Please fill out this template to help reviewers.
-->

## Description

<!-- Describe what this PR does and why. -->

## Related Issue

<!-- Link the issue this PR addresses, e.g. "Closes #42" -->
Closes #

## Type of Change

- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that changes existing behavior)
- [ ] Documentation update
- [ ] Refactor / tooling / CI

## Checklist

<!-- These match our CI gates — please run them locally before submitting. -->

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets --features testutils -- -D warnings` passes
- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test --workspace --features testutils` passes
- [ ] I added tests covering my changes (success **and** error paths where applicable)
- [ ] I updated documentation (README / doc comments) if behavior changed
- [ ] My commits have clear messages

## Security Considerations

<!-- For payment/financial logic, confirm you've considered these. Delete if not applicable. -->

- [ ] Inputs are validated (amounts, addresses, percentages, bounds)
- [ ] Error paths leave no inconsistent state
- [ ] Authorization (`require_auth`) is applied where needed

## Additional Notes

<!-- Anything reviewers should know: trade-offs, follow-ups, screenshots, etc. -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Nothing here has been cut into a release yet. Each entry links to the issue that
drives it, and in-progress work is marked as such until it lands.

### Added
- `require_auth` on `Payments::complete`, so only the payer can settle a
  payment it started ([#29](https://github.com/LintError/orionpay-contract/issues/29))
  (in progress).

### Fixed
- Split-payment IDs no longer collide with bulk-payment IDs
  ([#31](https://github.com/LintError/orionpay-contract/issues/31))
  (in progress).

### BREAKING
Several in-flight changes alter the on-chain ABI and will bump the version
when they land: ID format, `String` to `Address` conversions, storage-layout,
and token transfers.

## [0.1.0]

Initial state of the contract. `OrionPay::version()` / `OrionPayStellar::version()`
reported `0.1.0` (and `0.1.0-stellar` for the Stellar wrapper) with no release
notes; this changelog starts tracking changes from `[Unreleased]` onward.

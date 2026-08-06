# Security Policy

## Supported Versions

We are currently in early development. Security updates will be provided for the latest commit on the `main` branch.

| Version | Supported          |
| ------- | ------------------ |
| main    | :white_check_mark: |

## Reporting a Vulnerability

**OrionPay is a payment contract handling financial transactions.** Security vulnerabilities could lead to loss of funds or unauthorized access. We take security seriously and appreciate responsible disclosure.

### How to Report

**Do NOT open a public issue for security vulnerabilities.**

Instead, please report security issues via one of these methods:

1. **Email** (preferred): Send details to the repository maintainer's email listed in git commits
2. **GitHub Security Advisory**: Use the "Security" tab → "Report a vulnerability" (if you have access to this repository)

### What to Include

Please provide:

- **Description** of the vulnerability
- **Steps to reproduce** or proof-of-concept
- **Potential impact** (e.g., unauthorized payment, locked funds, bypassed validation)
- **Affected versions/commits** if known
- **Suggested fix** if you have one

### What to Expect

- **Acknowledgment** within 48 hours
- **Assessment** of severity and impact within 1 week
- **Regular updates** on progress toward a fix
- **Credit** in the security advisory once the issue is resolved (if you wish)

### Disclosure Timeline

We follow coordinated disclosure:

1. We confirm the issue and develop a fix
2. We test the fix and prepare a release
3. We publish the fix and a security advisory
4. Public disclosure happens **after** the fix is released

We aim to resolve critical issues within 30 days. We will keep you informed throughout the process.

## Security Considerations for Contributors

When contributing to OrionPay:

- **Validate all inputs** — amounts, addresses, percentages, array bounds
- **Check for arithmetic overflow/underflow** — use checked math where appropriate
- **Test error paths** — ensure failures don't leave inconsistent state
- **Never ignore `Result` types** — handle or propagate errors explicitly
- **Require authorization** where needed — use `Address::require_auth()` appropriately
- **Test edge cases** — zero amounts, empty vectors, boundary values
- **Storage safety** — ensure storage operations maintain consistency

### Common Vulnerability Classes in Smart Contracts

Be aware of:

- **Reentrancy** (though less common in Soroban than Ethereum)
- **Integer overflow/underflow**
- **Unauthorized access** to privileged functions
- **State inconsistency** after partial failures
- **Timestamp manipulation** (if using `env.ledger().timestamp()` for logic)
- **Denial of service** via unbounded loops or storage exhaustion

## Out of Scope

The following are **not** security vulnerabilities:

- Issues in third-party tools or dependencies (report to their maintainers)
- Missing features or design suggestions (use feature requests)
- Gas optimization opportunities (use standard issues)
- Theoretical attacks with no practical impact

## Questions?

For general security questions (not vulnerability reports), open a discussion or issue tagged `security`.

Thank you for helping keep OrionPay secure!

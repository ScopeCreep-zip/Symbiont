# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in Symbiont, please report it responsibly:

1. **Do NOT** open a public GitHub issue for security vulnerabilities
2. Email security concerns to: [security@example.com] (replace with actual contact)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 1 week
- **Fix Timeline**: Depends on severity
  - Critical: 24-72 hours
  - High: 1 week
  - Medium: 2-4 weeks
  - Low: Next release

## Security Considerations

### Cryptographic Operations

Symbiont uses:
- **Ed25519** for digital signatures (via `ed25519-dalek`)
- **BLAKE3** for cryptographic hashing

### Trust Model Assumptions

The protocol assumes:
- Node IDs are cryptographically bound to key pairs
- Signatures are verified before processing messages
- Network transport provides basic integrity (TLS/encryption at transport layer)

### Known Limitations

1. **Sybil Resistance**: Requires external identity binding (e.g., stake, social graph)
2. **Eclipse Attacks**: Nodes should maintain diverse connections
3. **Timing Attacks**: Not currently hardened against timing side-channels

### Security Best Practices

When deploying Symbiont:

1. **Key Management**
   - Store private keys securely
   - Rotate keys periodically
   - Use hardware security modules in production

2. **Network Security**
   - Use TLS for all network communications
   - Implement rate limiting
   - Monitor for anomalous patterns

3. **Operational Security**
   - Keep dependencies updated
   - Run `cargo audit` regularly
   - Enable logging and monitoring

## Dependency Auditing

We recommend running:

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit
cargo audit

# Install cargo-deny
cargo install cargo-deny

# Check for issues
cargo deny check
```

## Security Updates

Security updates will be announced via:
- GitHub Security Advisories
- Release notes
- [Mailing list if applicable]

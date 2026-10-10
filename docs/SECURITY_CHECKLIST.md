# Security Checklist

- [ ] Program IDs synchronized with generated keypairs.
- [ ] Core PDA deterministic.
- [ ] Core mutation requires stored authority.
- [ ] Duplicate Core initialization fails.
- [ ] Version zero rejected.
- [ ] Attestation PDA deterministic.
- [ ] Attestation schema/version zero rejected.
- [ ] All-zero evidence hash rejected.
- [ ] Duplicate identical attestation fails.
- [ ] Only stored issuer can revoke.
- [ ] Revocation preserves original evidence hash.
- [ ] Release overflow checks enabled.
- [ ] Clippy passes with warnings denied.
- [ ] Upgrade authorities documented before devnet.
- [ ] No mainnet deployment before separate security review.

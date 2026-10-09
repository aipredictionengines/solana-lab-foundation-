# Architecture

## Foundation order

```text
01 — Core / Rust SDK
        ↓
04 — Attestation / Proof Engine
        ↓
#001 Risk Passport
```

## Core
The Core program establishes reusable conventions for:
- deterministic PDAs;
- authority;
- versioning;
- timestamps;
- state transitions;
- events and domain errors.

## Attestation Engine
The Attestation program records a signed commitment to a 32-byte evidence hash.

```text
OFF-CHAIN REPORT
      ↓ SHA-256
EVIDENCE HASH
      ↓
ISSUER SIGNATURE
      ↓
ATTESTATION PDA
      ↓
PUBLIC VERIFICATION
```

The chain proves the commitment and issuer. It does not independently prove
that the underlying report is true.

Evidence is immutable. A changed report creates a new hash and a new
attestation. Existing attestations may transition from Active to Revoked.

## Risk Passport
Risk Passport will be the first public product built on this foundation.

Product pricing is deliberately outside the foundation programs.

# Solana Lab Foundation

Reusable Rust/Solana foundation for Solana Lab.

> **Status:** Foundation v0.1 — build/test gate in progress.

## Build order

```text
Core / Rust SDK v0.1
        ↓
Attestation Engine v0.1
        ↓
Compile → 16 Tests → Clippy → Local Validator → Devnet
        ↓
PASS
        ↓
Risk Passport v0.1
```

## Programs

### Lab Core
Shared conventions for deterministic PDAs, authority, versioning, timestamps,
events and state transitions.

### Attestation Engine
Stores verifiable commitments to SHA-256 evidence hashes. It is the TRUST
primitive that Risk Passport, EventProof, BuilderProof and later products can
reuse.

## Risk Passport commercial model
Initial paid production-use hypothesis: **$0.50 per scan/proof**.

Paying to use Risk Passport does **not** grant rights to copy, fork, self-host,
modify, redistribute or commercialize Solana Lab source code.

## License
The repository is intentionally public and source-visible, but **not open
source**. All rights are reserved unless a module is explicitly relicensed.

See `LICENSE` and `docs/COMMERCIAL_POLICY_V0_1.md`.

## Local development
Official Solana/Anchor installation currently supports macOS, Linux and
Windows through WSL.

After installing the toolchain:

```bash
yarn install
anchor keys sync
anchor build
cargo clippy --workspace --all-targets -- -D warnings
anchor test
```

Do not deploy to devnet until the local gates pass.

See `docs/TEST_GATES.md` and `docs/STATUS.md`.

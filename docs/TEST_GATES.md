# Build and Release Gates

Foundation v0.1 is not PASS because source exists. It is PASS only after:

```text
COMPILE
  ↓
16 TESTS
  ↓
CLIPPY
  ↓
LOCAL VALIDATOR
  ↓
DEVNET
  ↓
PASS / FAIL
```

## Compile
- `anchor build` succeeds.

## 16 Tests
- all 16 Foundation integration tests pass;
- no test is removed merely to obtain green CI.

## Clippy
- `cargo clippy --workspace --all-targets -- -D warnings`.

## Local Validator
- full transaction lifecycle executes against a local validator.

## Devnet
- both program IDs are synchronized with deployment keypairs;
- both programs deploy;
- Core config PDA can be read;
- Attestation can be created and revoked;
- transaction signatures and deployed IDs are recorded in `docs/STATUS.md`.

## PASS
Only after all previous gates pass.

Risk Passport v0.1 is unlocked only after Foundation PASS.

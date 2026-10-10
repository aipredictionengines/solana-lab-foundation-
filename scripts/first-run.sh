#!/usr/bin/env bash
set -euo pipefail

echo "== Solana Lab Foundation =="

rustc --version
cargo --version
solana --version
anchor --version
node --version
yarn --version

echo "== Install JS dependencies =="
yarn install

echo "== Synchronize program IDs =="
anchor keys sync

echo "== Compile =="
anchor build

echo "== Clippy =="
cargo clippy --workspace --all-targets -- -D warnings

echo "== 16 tests / local validator =="
anchor test

echo "Local gates complete. Review docs/TEST_GATES.md before devnet."

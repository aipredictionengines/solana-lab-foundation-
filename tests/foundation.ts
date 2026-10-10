import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { createHash } from "crypto";
import { assert } from "chai";
import { LabCore } from "../target/types/lab_core";
import { AttestationEngine } from "../target/types/attestation_engine";

async function expectFailure(fn: () => Promise<unknown>) {
  let failed = false;
  try {
    await fn();
  } catch {
    failed = true;
  }
  assert.isTrue(failed, "expected transaction to fail");
}

describe("Solana Lab Foundation v0.1 — 16 tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const core = anchor.workspace.labCore as Program<LabCore>;
  const attest =
    anchor.workspace.attestationEngine as Program<AttestationEngine>;

  const attacker = Keypair.generate();

  const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("lab-config")],
    core.programId
  );

  const subject = Keypair.generate().publicKey;
  const payload = Buffer.from(JSON.stringify({
    product: "risk-passport",
    schema: 1,
    subject: subject.toBase58(),
    result: "TEST_ONLY"
  }));
  const evidenceHash = Array.from(
    createHash("sha256").update(payload).digest()
  );
  const zeroHash = new Array(32).fill(0);

  const deriveAttestation = (hash: number[]) =>
    PublicKey.findProgramAddressSync(
      [
        Buffer.from("attestation"),
        provider.wallet.publicKey.toBuffer(),
        subject.toBuffer(),
        Buffer.from(hash),
      ],
      attest.programId
    )[0];

  const attestationPda = deriveAttestation(evidenceHash);

  it("01 derives the Core PDA deterministically", async () => {
    const [again] = PublicKey.findProgramAddressSync(
      [Buffer.from("lab-config")],
      core.programId
    );
    assert.equal(again.toBase58(), configPda.toBase58());
  });

  it("02 rejects Core version zero", async () => {
    await expectFailure(() =>
      core.methods.initialize(0).accountsStrict({
        authority: provider.wallet.publicKey,
        config: configPda,
        systemProgram: SystemProgram.programId,
      }).rpc()
    );
  });

  it("03 initializes Core state", async () => {
    await core.methods.initialize(1).accountsStrict({
      authority: provider.wallet.publicKey,
      config: configPda,
      systemProgram: SystemProgram.programId,
    }).rpc();

    const config = await core.account.labConfig.fetch(configPda);
    assert.equal(
      config.authority.toBase58(),
      provider.wallet.publicKey.toBase58()
    );
    assert.equal(config.version, 1);
  });

  it("04 initializes Core as Active", async () => {
    const config = await core.account.labConfig.fetch(configPda);
    assert.property(config.status, "active");
  });

  it("05 rejects duplicate Core initialization", async () => {
    await expectFailure(() =>
      core.methods.initialize(1).accountsStrict({
        authority: provider.wallet.publicKey,
        config: configPda,
        systemProgram: SystemProgram.programId,
      }).rpc()
    );
  });

  it("06 allows authority to pause Core", async () => {
    await core.methods.setStatus({ paused: {} }).accountsStrict({
      authority: provider.wallet.publicKey,
      config: configPda,
    }).rpc();

    const config = await core.account.labConfig.fetch(configPda);
    assert.property(config.status, "paused");
  });

  it("07 allows authority to reactivate Core", async () => {
    await core.methods.setStatus({ active: {} }).accountsStrict({
      authority: provider.wallet.publicKey,
      config: configPda,
    }).rpc();

    const config = await core.account.labConfig.fetch(configPda);
    assert.property(config.status, "active");
  });

  it("08 rejects unauthorized Core status changes", async () => {
    await expectFailure(() =>
      core.methods.setStatus({ paused: {} }).accountsStrict({
        authority: attacker.publicKey,
        config: configPda,
      }).signers([attacker]).rpc()
    );
  });

  it("09 derives Attestation PDA deterministically", async () => {
    const again = deriveAttestation(evidenceHash);
    assert.equal(again.toBase58(), attestationPda.toBase58());
  });

  it("10 rejects schema zero", async () => {
    await expectFailure(() =>
      attest.methods.createAttestation(
        subject,
        evidenceHash,
        0,
        1
      ).accountsStrict({
        issuer: provider.wallet.publicKey,
        attestation: attestationPda,
        systemProgram: SystemProgram.programId,
      }).rpc()
    );
  });

  it("11 rejects version zero", async () => {
    await expectFailure(() =>
      attest.methods.createAttestation(
        subject,
        evidenceHash,
        1,
        0
      ).accountsStrict({
        issuer: provider.wallet.publicKey,
        attestation: attestationPda,
        systemProgram: SystemProgram.programId,
      }).rpc()
    );
  });

  it("12 rejects an all-zero evidence hash", async () => {
    const zeroPda = deriveAttestation(zeroHash);
    await expectFailure(() =>
      attest.methods.createAttestation(
        subject,
        zeroHash,
        1,
        1
      ).accountsStrict({
        issuer: provider.wallet.publicKey,
        attestation: zeroPda,
        systemProgram: SystemProgram.programId,
      }).rpc()
    );
  });

  it("13 creates an Active evidence attestation", async () => {
    await attest.methods.createAttestation(
      subject,
      evidenceHash,
      1,
      1
    ).accountsStrict({
      issuer: provider.wallet.publicKey,
      attestation: attestationPda,
      systemProgram: SystemProgram.programId,
    }).rpc();

    const account = await attest.account.attestation.fetch(attestationPda);
    assert.equal(account.subject.toBase58(), subject.toBase58());
    assert.deepEqual(Array.from(account.evidenceHash), evidenceHash);
    assert.property(account.status, "active");
  });

  it("14 rejects duplicate identical attestations", async () => {
    await expectFailure(() =>
      attest.methods.createAttestation(
        subject,
        evidenceHash,
        1,
        1
      ).accountsStrict({
        issuer: provider.wallet.publicKey,
        attestation: attestationPda,
        systemProgram: SystemProgram.programId,
      }).rpc()
    );
  });

  it("15 rejects revocation by a non-issuer", async () => {
    await expectFailure(() =>
      attest.methods.revokeAttestation().accountsStrict({
        issuer: attacker.publicKey,
        attestation: attestationPda,
      }).signers([attacker]).rpc()
    );
  });

  it("16 issuer can revoke without mutating evidence", async () => {
    await attest.methods.revokeAttestation().accountsStrict({
      issuer: provider.wallet.publicKey,
      attestation: attestationPda,
    }).rpc();

    const account = await attest.account.attestation.fetch(attestationPda);
    assert.property(account.status, "revoked");
    assert.deepEqual(Array.from(account.evidenceHash), evidenceHash);
    assert.isNotNull(account.revokedAt);
  });
});

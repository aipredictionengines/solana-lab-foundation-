use anchor_lang::prelude::*;

declare_id!("2UeTJJi7CkpSg1AsgQ3RV8aPfW2XVh7LqAnsfmoqZ9JN");

pub const ATTESTATION_SEED: &[u8] = b"attestation";

#[program]
pub mod attestation_engine {
    use super::*;

    pub fn create_attestation(
        ctx: Context<CreateAttestation>,
        subject: Pubkey,
        evidence_hash: [u8; 32],
        schema_id: u16,
        version: u16,
    ) -> Result<()> {
        require!(schema_id > 0, AttestationError::InvalidSchema);
        require!(version > 0, AttestationError::InvalidVersion);
        require!(
            evidence_hash.iter().any(|byte| *byte != 0),
            AttestationError::EmptyEvidenceHash
        );

        let now = Clock::get()?.unix_timestamp;
        let attestation = &mut ctx.accounts.attestation;

        attestation.issuer = ctx.accounts.issuer.key();
        attestation.subject = subject;
        attestation.evidence_hash = evidence_hash;
        attestation.schema_id = schema_id;
        attestation.version = version;
        attestation.status = AttestationStatus::Active;
        attestation.bump = ctx.bumps.attestation;
        attestation.created_at = now;
        attestation.revoked_at = None;

        emit!(AttestationCreated {
            attestation: attestation.key(),
            issuer: attestation.issuer,
            subject,
            evidence_hash,
            schema_id,
            version,
            created_at: now,
        });

        Ok(())
    }

    pub fn revoke_attestation(ctx: Context<RevokeAttestation>) -> Result<()> {
        let attestation = &mut ctx.accounts.attestation;

        require!(
            attestation.status == AttestationStatus::Active,
            AttestationError::AlreadyRevoked
        );

        let now = Clock::get()?.unix_timestamp;
        attestation.status = AttestationStatus::Revoked;
        attestation.revoked_at = Some(now);

        emit!(AttestationRevoked {
            attestation: attestation.key(),
            issuer: attestation.issuer,
            revoked_at: now,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(
    subject: Pubkey,
    evidence_hash: [u8; 32],
    schema_id: u16,
    version: u16
)]
pub struct CreateAttestation<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,

    #[account(
        init,
        payer = issuer,
        space = 8 + Attestation::INIT_SPACE,
        seeds = [
            ATTESTATION_SEED,
            issuer.key().as_ref(),
            subject.as_ref(),
            evidence_hash.as_ref(),
        ],
        bump
    )]
    pub attestation: Account<'info, Attestation>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeAttestation<'info> {
    pub issuer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            ATTESTATION_SEED,
            attestation.issuer.as_ref(),
            attestation.subject.as_ref(),
            attestation.evidence_hash.as_ref(),
        ],
        bump = attestation.bump,
        has_one = issuer
    )]
    pub attestation: Account<'info, Attestation>,
}

#[account]
#[derive(InitSpace)]
pub struct Attestation {
    pub issuer: Pubkey,
    pub subject: Pubkey,
    pub evidence_hash: [u8; 32],
    pub schema_id: u16,
    pub version: u16,
    pub status: AttestationStatus,
    pub bump: u8,
    pub created_at: i64,
    pub revoked_at: Option<i64>,
}

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    InitSpace
)]
pub enum AttestationStatus {
    Active,
    Revoked,
}

#[event]
pub struct AttestationCreated {
    pub attestation: Pubkey,
    pub issuer: Pubkey,
    pub subject: Pubkey,
    pub evidence_hash: [u8; 32],
    pub schema_id: u16,
    pub version: u16,
    pub created_at: i64,
}

#[event]
pub struct AttestationRevoked {
    pub attestation: Pubkey,
    pub issuer: Pubkey,
    pub revoked_at: i64,
}

#[error_code]
pub enum AttestationError {
    #[msg("Schema ID must be greater than zero.")]
    InvalidSchema,
    #[msg("Version must be greater than zero.")]
    InvalidVersion,
    #[msg("Evidence hash cannot be all zeroes.")]
    EmptyEvidenceHash,
    #[msg("Attestation has already been revoked.")]
    AlreadyRevoked,
}

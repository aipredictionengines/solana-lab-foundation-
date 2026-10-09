use anchor_lang::prelude::*;

declare_id!("2DAzN45Mzr41VPAGbyakKdmVykU8x92cG2Fe3geb6BGt");

pub const LAB_CONFIG_SEED: &[u8] = b"lab-config";

#[program]
pub mod lab_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, version: u16) -> Result<()> {
        require!(version > 0, LabCoreError::InvalidVersion);

        let now = Clock::get()?.unix_timestamp;
        let config = &mut ctx.accounts.config;

        config.authority = ctx.accounts.authority.key();
        config.version = version;
        config.status = LabStatus::Active;
        config.bump = ctx.bumps.config;
        config.created_at = now;
        config.updated_at = now;

        emit!(CoreInitialized {
            config: config.key(),
            authority: config.authority,
            version,
            created_at: now,
        });

        Ok(())
    }

    pub fn set_status(ctx: Context<SetStatus>, status: LabStatus) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let config = &mut ctx.accounts.config;
        let old_status = config.status;

        config.status = status;
        config.updated_at = now;

        emit!(CoreStatusChanged {
            config: config.key(),
            authority: config.authority,
            old_status,
            new_status: status,
            updated_at: now,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + LabConfig::INIT_SPACE,
        seeds = [LAB_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, LabConfig>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetStatus<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [LAB_CONFIG_SEED],
        bump = config.bump,
        has_one = authority @ LabCoreError::Unauthorized
    )]
    pub config: Account<'info, LabConfig>,
}

#[account]
#[derive(InitSpace)]
pub struct LabConfig {
    pub authority: Pubkey,
    pub version: u16,
    pub status: LabStatus,
    pub bump: u8,
    pub created_at: i64,
    pub updated_at: i64,
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
pub enum LabStatus {
    Active,
    Paused,
}

#[event]
pub struct CoreInitialized {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub version: u16,
    pub created_at: i64,
}

#[event]
pub struct CoreStatusChanged {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub old_status: LabStatus,
    pub new_status: LabStatus,
    pub updated_at: i64,
}

#[error_code]
pub enum LabCoreError {
    #[msg("Version must be greater than zero.")]
    InvalidVersion,
    #[msg("Signer is not the configured authority.")]
    Unauthorized,
}

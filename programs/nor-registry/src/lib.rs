use anchor_lang::prelude::*;

pub const MAX_NAME_LEN: usize = 32;
pub const MAX_TTL_SECONDS: i64 = 31_536_000;

declare_id!("NorReg1111111111111111111111111111111111111");

#[program]
pub mod nor_registry {
    use super::*;

    pub fn initialize_registry(ctx: Context<InitializeRegistry>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.bump = ctx.bumps.registry;
        Ok(())
    }

    pub fn register_name(
        ctx: Context<RegisterName>,
        name: String,
        target: Pubkey,
        duration: i64,
    ) -> Result<()> {
        validate_name(&name)?;
        require!((1..=MAX_TTL_SECONDS).contains(&duration), NorError::InvalidDuration);

        let record = &mut ctx.accounts.record;
        record.registry = ctx.accounts.registry.key();
        record.owner = ctx.accounts.owner.key();
        record.target = target;
        record.name = name;
        record.expires_at = Clock::get()?.unix_timestamp
            .checked_add(duration)
            .ok_or(NorError::ArithmeticOverflow)?;
        record.bump = ctx.bumps.record;
        Ok(())
    }

    pub fn renew_name(ctx: Context<RenewName>, additional_duration: i64) -> Result<()> {
        require!((1..=MAX_TTL_SECONDS).contains(&additional_duration), NorError::InvalidDuration);
        let record = &mut ctx.accounts.record;
        let now = Clock::get()?.unix_timestamp;
        let base = record.expires_at.max(now);
        record.expires_at = base
            .checked_add(additional_duration)
            .ok_or(NorError::ArithmeticOverflow)?;
        Ok(())
    }

    pub fn transfer_name(ctx: Context<TransferName>, new_owner: Pubkey) -> Result<()> {
        ctx.accounts.record.owner = new_owner;
        Ok(())
    }
}

fn validate_name(name: &str) -> Result<()> {
    require!(!name.is_empty() && name.len() <= MAX_NAME_LEN, NorError::InvalidName);
    require!(name.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'), NorError::InvalidName);
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(init, payer = authority, space = 8 + Registry::INIT_SPACE, seeds = [b"registry"], bump)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct RegisterName<'info> {
    pub registry: Account<'info, Registry>,
    #[account(init, payer = owner, space = 8 + NameRecord::INIT_SPACE, seeds = [b"name", registry.key().as_ref(), name.as_bytes()], bump)]
    pub record: Account<'info, NameRecord>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RenewName<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, NameRecord>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferName<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, NameRecord>,
    pub owner: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Registry {
    pub authority: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct NameRecord {
    pub registry: Pubkey,
    pub owner: Pubkey,
    pub target: Pubkey,
    #[max_len(32)]
    pub name: String,
    pub expires_at: i64,
    pub bump: u8,
}

#[error_code]
pub enum NorError {
    #[msg("Name must be 1-32 lowercase ASCII characters, digits, or hyphens")]
    InvalidName,
    #[msg("Duration must be between one second and one year")]
    InvalidDuration,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}

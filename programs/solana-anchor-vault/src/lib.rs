use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("VauLt11111111111111111111111111111111111111");

#[program]
pub mod solana_anchor_vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        vault_bump: u8,
        lock_duration: i64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault_state;
        vault.initializer = ctx.accounts.initializer.key();
        vault.mint = ctx.accounts.mint.key();
        vault.vault_bump = vault_bump;
        vault.locked_amount = 0;
        vault.unlock_timestamp = Clock::get()?.unix_timestamp + lock_duration;
        vault.is_active = true;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(ctx.accounts.vault_state.is_active, VaultError::VaultInactive);

        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        let vault = &mut ctx.accounts.vault_state;
        vault.locked_amount = vault.locked_amount.checked_add(amount).unwrap();
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time >= ctx.accounts.vault_state.unlock_timestamp,
            VaultError::TimelockActive
        );
        require!(
            amount <= ctx.accounts.vault_state.locked_amount,
            VaultError::InsufficientFunds
        );

        let initializer_key = ctx.accounts.vault_state.initializer;
        let seeds = &[
            b"vault-authority".as_ref(),
            initializer_key.as_ref(),
            &[ctx.accounts.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.beneficiary_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds),
            amount,
        )?;

        let vault = &mut ctx.accounts.vault_state;
        vault.locked_amount = vault.locked_amount.checked_sub(amount).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(vault_bump: u8, lock_duration: i64)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = initializer,
        space = 8 + 32 + 32 + 1 + 8 + 8 + 1,
        seeds = [b"vault-state", initializer.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault_state: Account<'info, VaultState>,
    /// CHECK: PDA authority
    pub vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub beneficiary_token_account: Account<'info, TokenAccount>,
    pub beneficiary: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct VaultState {
    pub initializer: Pubkey,
    pub mint: Pubkey,
    pub vault_bump: u8,
    pub locked_amount: u64,
    pub unlock_timestamp: i64,
    pub is_active: bool,
}

#[error_code]
pub enum VaultError {
    #[msg("Vault is currently paused or inactive")]
    VaultInactive,
    #[msg("Unlock timelock has not expired yet")]
    TimelockActive,
    #[msg("Requested amount exceeds current vault balance")]
    InsufficientFunds,
}

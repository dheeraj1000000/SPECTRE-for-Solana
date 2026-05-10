//! TOK-023 safe bench fixture: fee-transfer scenario.
//!
//! Same shape as `fee-transfer-tok023-vuln.rs`, but the `deposit` handler
//! computes the Token-2022 transfer fee via `mint.calculate_fee(amount)` and
//! reduces the CPI amount before invoking `transfer_checked`. The program's
//! accounting then records the post-fee `net` rather than the gross
//! `amount`, so the destination's recorded balance matches the actual
//! credited balance.
//!
//! EXPECTED: TOK-023 does NOT fire on `Vault::deposit`.
//!
//! Detection trace:
//!   1. `is_anchor_program` is true.
//!   2. The instruction body contains `calculate_fee`, which is a
//!      fee-adjustment token. `body_is_fee_adjusted` returns true and the
//!      whole-handler suppression path takes effect before any mint-field
//!      resolution runs.
//!   3. No finding is emitted, even though the mint still carries the
//!      `TransferFeeConfig` extension and `token_2022::transfer_checked`
//!      is still called.

use anchor_lang::prelude::*;
use anchor_spl::token_2022;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("Vau1tFeeTrnsferTok023SafeFixture0000000000000");

#[program]
pub mod vault {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64, decimals: u8) -> Result<()> {
        // Compute the Token-2022 transfer fee for this mint at the current
        // amount, then subtract it so `net` is what the destination will
        // actually be credited.
        let fee = ctx
            .accounts
            .mint
            .calculate_fee(amount)
            .ok_or(ErrorCode::FeeOverflow)?;
        let net = amount.checked_sub(fee).ok_or(ErrorCode::FeeOverflow)?;

        let cpi_accounts = TransferChecked {
            from: ctx.accounts.from.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        token_2022::transfer_checked(cpi_ctx, net, decimals)?;

        ctx.accounts.vault_state.balance = ctx
            .accounts
            .vault_state
            .balance
            .checked_add(net)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub from: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        extensions::transfer_fee::transfer_fee_config,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub to: InterfaceAccount<'info, TokenAccount>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub vault_state: Account<'info, VaultState>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[account]
pub struct VaultState {
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("balance overflow")]
    Overflow,
    #[msg("fee calculation overflow")]
    FeeOverflow,
}

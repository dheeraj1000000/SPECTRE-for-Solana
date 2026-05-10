//! TOK-023 vulnerable bench fixture: fee-transfer scenario.
//!
//! Replicates the architectural shape from a manual audit. The `deposit`
//! handler performs a Token-2022 `transfer_checked` CPI on a mint that
//! declares the `TransferFeeConfig` extension, but it forwards the
//! user-supplied `amount` unchanged. Token-2022 will deduct the configured
//! transfer fee from the credited amount, so the program's accounting (which
//! treats `amount` as the post-transfer balance delta) silently over-credits
//! the destination by exactly the fee.
//!
//! EXPECTED: TOK-023 fires once on `Vault::deposit`.
//!
//! Detection trace:
//!   1. `is_anchor_program` is true (anchor program module present).
//!   2. The instruction body contains no fee-adjustment token
//!      (`fee_bps`, `transfer_fee`, `calculate_fee`, `fee.calculate`).
//!   3. The body contains `token_2022::transfer_checked`.
//!   4. The `mint:` named field resolves to `ctx.accounts.mint`.
//!   5. The companion `anchor:account-field:Deposit:mint` carries the
//!      `extensions::transfer_fee::transfer_fee_config` Anchor 0.30+
//!      constraint, so `mint_has_extension(.., TransferFeeConfig)` is true.
//!   6. Rule emits a HIGH-severity finding with confidence 0.82.

use anchor_lang::prelude::*;
use anchor_spl::token_2022;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("Vau1tFeeTrnsferTok023VuLnFixture000000000000");

#[program]
pub mod vault {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64, decimals: u8) -> Result<()> {
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
        // BUG: `amount` is forwarded verbatim. The TransferFeeConfig
        // extension on `mint` will cause the credited amount to be
        // `amount - fee`, but the program records `amount` as the
        // deposited balance below.
        token_2022::transfer_checked(cpi_ctx, amount, decimals)?;

        ctx.accounts.vault_state.balance = ctx
            .accounts
            .vault_state
            .balance
            .checked_add(amount)
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
}

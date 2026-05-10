// Bench fixture: NONCE-002 vulnerable (single-generation staleness).
//
// Replicates the `update_nonce` pattern from a manual audit: a privileged
// admin handler writes a user-defined `NonceAccount.nonce` counter without
// enforcing a strict monotonic floor (`require!(new > ctx.accounts.nonce_account.nonce, ...)`).
// A stale pre-signed transaction can replay an older `nonce` value and roll
// the counter sideways or backwards, defeating the per-event replay guard.
//
// EXPECTED: NONCE-002 fires once on `update_nonce`.
//   Layer 1 (privileged): `admin: Signer<'info>` -> admin-privileged.
//   Layer 2 (user-defined nonce surface): `nonce_account: Account<'info, NonceAccount>`
//     where `NonceAccount` is in USER_DEFINED_NONCE_STRUCTS and has a `nonce: u64`
//     counter field.
//   Writer suppression check: body assigns `ctx.accounts.nonce_account.nonce = ...`
//     so the handler is a writer, not a reader.
//   Layer 3 (floor missing): no `require!`/`require_gt!`/`require_gte!` macro
//     references `ctx.accounts.nonce_account.nonce` with `>` or `>=`. Rule fires.

use anchor_lang::prelude::*;

declare_id!("Nonce2vuln1111111111111111111111111111111");

#[program]
pub mod nonce_stale_vuln {
    use super::*;

    pub fn update_nonce(ctx: Context<UpdateNonce>, nonce: u64) -> Result<()> {
        // No floor check before the write.
        ctx.accounts.nonce_account.nonce = nonce;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateNonce<'info> {
    pub admin: Signer<'info>,
    #[account(mut)]
    pub nonce_account: Account<'info, NonceAccount>,
}

#[account]
pub struct NonceAccount {
    pub nonce: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("nonce mismatch")]
    NonceMismatch,
}

// Bench fixture: NONCE-002 safe (floor check enforced).
//
// Same `update_nonce` handler as the vulnerable single-gen fixture, with the
// strict monotonic floor restored: the new `nonce` argument must exceed the
// stored `ctx.accounts.nonce_account.nonce` before the write proceeds. A
// stale pre-signed transaction carrying an older nonce now fails the
// `require!` and cannot replay.
//
// EXPECTED: NONCE-002 does NOT fire on `update_nonce`.
//   Layer 1 + Layer 2 still match (privileged admin + user-defined NonceAccount surface).
//   Writer suppression check still passes (the body still writes the counter).
//   Layer 3 SUPPRESSES: `body_has_monotonic_guard_on(body, "nonce_account", "nonce")`
//     finds `require!(nonce > ctx.accounts.nonce_account.nonce, ...)` — a
//     `require!` whose paren-group contains both ` > ` and the qualified path
//     `ctx.accounts.nonce_account.nonce`. The guard is field-specific (keyed
//     on the same `nonce_account` trigger field that the handler writes), so
//     suppression applies and no finding is emitted.

use anchor_lang::prelude::*;

declare_id!("Nonce2safe11111111111111111111111111111111");

#[program]
pub mod nonce_stale_safe {
    use super::*;

    pub fn update_nonce(ctx: Context<UpdateNonce>, nonce: u64) -> Result<()> {
        require!(
            nonce > ctx.accounts.nonce_account.nonce,
            ErrorCode::NonceMismatch
        );
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

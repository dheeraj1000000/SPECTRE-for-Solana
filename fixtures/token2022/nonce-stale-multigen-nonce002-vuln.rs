// Bench fixture: NONCE-002 vulnerable (multi-generation variant).
//
// Replicates a multi-generation nonce pattern from a manual audit:
// the `NonceAccount` PDA carries TWO independent monotonic counters
// (`nonce` for the current epoch and `prev_nonce` for the predecessor
// generation), and TWO privileged writer handlers update them. Neither
// handler enforces a strict floor on the counter it writes, so each is
// independently replayable.
//
// EXPECTED: NONCE-002 fires TWICE.
//
//   Finding 1 — handler `update_nonce`:
//     Layer 1: admin-privileged.
//     Layer 2: `nonce_account: Account<'info, NonceAccount>`; `NonceAccount`
//       is in USER_DEFINED_NONCE_STRUCTS; counter field resolves to `nonce`
//       (first match in COUNTER_FIELD_NAMES order).
//     Writer check: body writes `ctx.accounts.nonce_account.nonce = ...`.
//     Layer 3: no `require!(... > ctx.accounts.nonce_account.nonce ...)`.
//     Dedup key: ("nonce_stale_multigen_vuln", "nonce_account", "update_nonce")
//       -> distinct from finding 2 because instruction_name differs.
//
//   Finding 2 — handler `update_prev_nonce`:
//     Layer 1: admin-privileged.
//     Layer 2: same `nonce_account` field (Layer 2 reuses the same trigger
//       field). NOTE: per the rule's `struct_counter_field`, the counter
//       resolves to `nonce` (first hit), so the Layer 3 check is keyed on
//       `ctx.accounts.nonce_account.nonce`.
//     Writer check: body writes `ctx.accounts.nonce_account.prev_nonce = ...`.
//       `instruction_writes_own_nonce_field` (gov_002.rs, promoted to
//       pub(crate)) treats any assignment whose LHS path begins with
//       `ctx.accounts.<trigger_field>.` as a write to the trigger field, so
//       this `prev_nonce` write registers as a write to the `nonce_account`
//       trigger. The writer-only suppression therefore does not apply.
//     Layer 3: no guard mentions `ctx.accounts.nonce_account.nonce` with
//       `>`/`>=`, so the floor is absent and the rule fires.
//     Dedup key: ("nonce_stale_multigen_vuln", "nonce_account", "update_prev_nonce")
//       -> distinct triple, second finding emitted.

use anchor_lang::prelude::*;

declare_id!("Nonce2multigenvuln11111111111111111111111");

#[program]
pub mod nonce_stale_multigen_vuln {
    use super::*;

    pub fn update_nonce(ctx: Context<UpdateNonce>, nonce: u64) -> Result<()> {
        // Generation N counter advanced with no floor.
        ctx.accounts.nonce_account.nonce = nonce;
        Ok(())
    }

    pub fn update_prev_nonce(ctx: Context<UpdatePrevNonce>, prev_nonce: u64) -> Result<()> {
        // Generation N-1 counter advanced with no floor.
        ctx.accounts.nonce_account.prev_nonce = prev_nonce;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateNonce<'info> {
    pub admin: Signer<'info>,
    #[account(mut)]
    pub nonce_account: Account<'info, NonceAccount>,
}

#[derive(Accounts)]
pub struct UpdatePrevNonce<'info> {
    pub admin: Signer<'info>,
    #[account(mut)]
    pub nonce_account: Account<'info, NonceAccount>,
}

#[account]
pub struct NonceAccount {
    pub nonce: u64,
    pub prev_nonce: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("nonce mismatch")]
    NonceMismatch,
    #[msg("prev nonce mismatch")]
    PrevNonceMismatch,
}

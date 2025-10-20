use anchor_lang::prelude::*;
use crate::pool_state::PoolState;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = authority, space = PoolState::LEN)]
    pub pool_state: Account<'info, PoolState>,

    /// CHECK: token accounts or mints are only stored as Pubkey references in PoolState
    pub token_a: UncheckedAccount<'info>,

    /// CHECK:
    pub token_b: UncheckedAccount<'info>,

    /// CHECK: LP mint account reference
    pub lp_mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

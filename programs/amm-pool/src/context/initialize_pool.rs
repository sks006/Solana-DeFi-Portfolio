use crate::pool_state::PoolState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = authority, space = PoolState::LEN)]
    pub pool_state: Account<'info, PoolState>,

    /// CHECK: Pool authority PDA
    #[account(seeds = [pool_state.key().as_ref()], bump)]
    pub pool_authority: SystemAccount<'info>,

    /// CHECK: token A mint
    pub token_a: UncheckedAccount<'info>,

    /// CHECK: token B mint
    pub token_b: UncheckedAccount<'info>,

    /// CHECK: LP mint account
    pub lp_mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

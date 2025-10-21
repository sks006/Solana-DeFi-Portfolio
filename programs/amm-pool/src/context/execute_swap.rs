use crate::pool_state::PoolState;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct ExecuteSwap<'info> {
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,

    /// CHECK: Pool authority PDA
    #[account(seeds = [pool_state.key().as_ref()], bump)]
    pub pool_authority: AccountInfo<'info>,

    /// user token source (token A)
    #[account(mut)]
    pub user_source: Account<'info, TokenAccount>,

    /// user token destination (token B)
    #[account(mut)]
    pub user_destination: Account<'info, TokenAccount>,

    /// pool's token A vault
    #[account(mut)]
    pub pool_vault_a: Account<'info, TokenAccount>,

    /// pool's token B vault
    #[account(mut)]
    pub pool_vault_b: Account<'info, TokenAccount>,

    /// token program
    pub token_program: Program<'info, Token>,

    pub authority: Signer<'info>,
}

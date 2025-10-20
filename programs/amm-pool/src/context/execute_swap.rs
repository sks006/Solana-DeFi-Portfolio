use anchor_lang::prelude::*;
use crate::pool_state::PoolState;

#[derive(Accounts)]
pub struct ExecuteSwap<'info> {
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,

    /// user token source (e.g., user's token A associated token account) — payer of amount_in
    #[account(mut)]
    pub user_source: AccountInfo<'info>,

    /// user token destination (e.g., user's token B ATA) — receives amount_out
    #[account(mut)]
    pub user_destination: AccountInfo<'info>,

    /// pool's token A vault
    #[account(mut)]
    pub pool_vault_a: AccountInfo<'info>,

    /// pool's token B vault
    #[account(mut)]
    pub pool_vault_b: AccountInfo<'info>,

    /// token program
    pub token_program: Program<'info, Token>,

    pub authority: Signer<'info>,
}

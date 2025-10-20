use anchor_lang::prelude::*;

#[account]
pub struct PoolState {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub lp_mint: Pubkey,
    pub fee_rate_bps: u64,
    pub total_liquidity: u64,
    pub authority: Pubkey,
}

impl PoolState {
    // discriminator (8) + 4 pubkeys (32*4) + 2 u64s (8*2)
    pub const LEN: usize = 8 + (32 * 4) + (8 * 2);

    pub fn initialize(ctx: Context<crate::context::InitializePool>, fee_rate_bps: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool_state;
        pool.token_a = ctx.accounts.token_a.key();
        pool.token_b = ctx.accounts.token_b.key();
        pool.lp_mint = ctx.accounts.lp_mint.key();
        pool.fee_rate_bps = fee_rate_bps;
        pool.total_liquidity = 0;
        pool.authority = ctx.accounts.authority.key();
        Ok(())
    }
}

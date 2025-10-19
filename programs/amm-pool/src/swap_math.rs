use anchor_lang::prelude::*;
use crate::pool_state::Pool;

pub fn swap(ctx: Context<ExecuteSwap>, amount_in: u64, min_out: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    let amount_in_with_fee = amount_in * (10_000 - pool.fee_bps as u64) / 10_000;
    let amount_out = pool.reserve_b * amount_in_with_fee / (pool.reserve_a + amount_in_with_fee);

    require!(amount_out >= min_out, CustomError::SlippageExceeded);

    pool.reserve_a += amount_in;
    pool.reserve_b -= amount_out;

    Ok(())
}

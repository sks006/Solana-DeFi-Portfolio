use anchor_lang::prelude::*;

pub mod context;
pub mod fees;
pub mod pool_state;
pub mod swap_math;

use context::*;

// Use Anchor's standard placeholder - Anchor will replace this during build
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod amm_pool {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, fee_rate_bps: u64) -> Result<()> {
        pool_state::PoolState::initialize(ctx, fee_rate_bps)
    }

    pub fn execute_swap(ctx: Context<ExecuteSwap>, amount_in: u64, min_out: u64) -> Result<()> {
        swap_math::execute_swap(ctx, amount_in, min_out)
    }
}

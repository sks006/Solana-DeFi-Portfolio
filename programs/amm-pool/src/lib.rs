use anchor_lang::prelude::*;

pub mod pool_state;
pub mod swap_math;
pub mod fees;

#[program]
pub mod amm_pool {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, params: PoolParams) -> Result<()> {
        pool_state::initialize(ctx, params)
    }

    pub fn execute_swap(ctx: Context<ExecuteSwap>, amount_in: u64) -> Result<()> {
        swap_math::swap(ctx, amount_in)
    }
}

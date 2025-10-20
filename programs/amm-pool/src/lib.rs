use anchor_lang::prelude::*;

pub mod pool_state;
pub mod swap_math;
pub mod fees;
pub mod context;

use context::*;

declare_id!("4nvSCrLZjCCDum1Ccg59Wwev2DA3CJoHXL3UXXEg7bdN");

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

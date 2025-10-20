use anchor_lang::prelude::*;

pub mod state;
pub mod processor;
pub mod types;
pub mod context;

use context::*;

declare_id!("Port111111111111111111111111111111111111111");

#[program]
pub mod portfolio_program {
    use super::*;

    pub fn update_position(ctx: Context<UpdatePosition>, pnl: i64) -> Result<()> {
        processor::update_position(ctx, pnl)
    }

    pub fn record_trade(ctx: Context<RecordTrade>, trade_id: u64) -> Result<()> {
        processor::record_trade(ctx, trade_id)
    }
}

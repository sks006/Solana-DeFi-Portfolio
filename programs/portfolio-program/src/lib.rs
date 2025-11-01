use anchor_lang::prelude::*;

pub mod context;
pub mod processor;
pub mod state;


use context::*;

// Use the same placeholder - Anchor will generate unique IDs for each program
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

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

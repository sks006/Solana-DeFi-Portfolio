use anchor_lang::prelude::*;

pub mod state;
pub mod instruction;
pub mod processor;

#[program]
pub mod portfolio_program {
    use super::*;

    pub fn update_position(ctx: Context<UpdatePosition>, asset: Pubkey, delta: i64) -> Result<()> {
        processor::update(ctx, asset, delta)
    }

    pub fn record_trade(ctx: Context<RecordTrade>, trade: TradeData) -> Result<()> {
        processor::record(ctx, trade)
    }
}

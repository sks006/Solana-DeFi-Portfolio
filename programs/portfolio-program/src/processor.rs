use anchor_lang::prelude::*;
use crate::state::Position;

pub fn update_position(ctx: Context<crate::context::UpdatePosition>, pnl_delta: i64) -> Result<()> {
    let pos = &mut ctx.accounts.position;
    // Basic authorization: ensure signer is the owner
    require_keys_eq!(pos.owner, ctx.accounts.authority.key(), ErrorCode::Unauthorized);

    // Update PnL safely
    pos.pnl = pos.pnl.checked_add(pnl_delta).ok_or(error!(ErrorCode::MathOverflow))?;

    Ok(())
}

pub fn record_trade(ctx: Context<crate::context::RecordTrade>, trade_id: u64) -> Result<()> {
    let pos = &mut ctx.accounts.position;

    // simple increment of trade_count
    pos.trade_count = pos.trade_count.checked_add(1).ok_or(error!(ErrorCode::MathOverflow))?;

    // Optionally, you would append a trade record account or emit an event - here we just increment a counter
    // Emitting an event:
    emit!(TradeRecorded {
        owner: pos.owner,
        trade_id,
        trade_number: pos.trade_count,
    });

    Ok(())
}

#[event]
pub struct TradeRecorded {
    pub owner: Pubkey,
    pub trade_id: u64,
    pub trade_number: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Unauthorized")]
    Unauthorized,
}

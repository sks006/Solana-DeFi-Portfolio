// record_trade.rs
use anchor_lang::prelude::*;
use crate::state::Position;

#[derive(Accounts)]
pub struct RecordTrade<'info> {
    #[account(mut)]
    pub position: Account<'info, Position>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
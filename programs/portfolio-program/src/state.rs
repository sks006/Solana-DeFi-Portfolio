use anchor_lang::prelude::*;

#[account]
pub struct Position {
    pub owner: Pubkey,
    pub pnl: i64,
    pub trade_count: u64,
}

impl Position {
    // discriminator (8) + owner (32) + pnl (8) + trade_count (8)
    pub const LEN: usize = 8 + 32 + 8 + 8;
}

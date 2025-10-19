#[account]
pub struct Position {
    pub owner: Pubkey,
    pub asset: Pubkey,
    pub amount: i64,
    pub pnl: i64,
}

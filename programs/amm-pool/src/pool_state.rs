use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_bps: u16,
    pub authority: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PoolParams {
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_bps: u16,
}

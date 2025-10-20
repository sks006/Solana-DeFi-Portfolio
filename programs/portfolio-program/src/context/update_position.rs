// update_position.rs
use anchor_lang::prelude::*;
use crate::state::Position;

#[derive(Accounts)]
pub struct UpdatePosition<'info> {
    #[account(mut, has_one = owner)]
    pub position: Account<'info, Position>,
    pub owner: Signer<'info>,
    /// CHECK: authority may be same as owner or program admin
    pub authority: Signer<'info>,
}
use crate::state::Position;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdatePosition<'info> {
    #[account(mut, has_one = owner)]
    pub position: Account<'info, Position>,
    pub owner: Signer<'info>,
}

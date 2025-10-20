use anchor_lang::prelude::*;
use crate::fees::apply_fee;

pub fn execute_swap(ctx: Context<crate::context::ExecuteSwap>, amount_in: u64, min_out: u64) -> Result<()> {
    // 1) Read pool vault balances (we assume token vault accounts are passed and contain accurate balances)
    let pool_vault_a_info = &ctx.accounts.pool_vault_a;
    let pool_vault_b_info = &ctx.accounts.pool_vault_b;
    // For brevity, assume we can read balances as u64 by deserializing token account data
    let vault_a_balance = get_token_balance(pool_vault_a_info)? as u128;
    let vault_b_balance = get_token_balance(pool_vault_b_info)? as u128;

    // 2) Apply fee to input amount
    let fee_rate = ctx.accounts.pool_state.fee_rate_bps; // u64 bps
    let (effective_in, fee_amount) = apply_fee(amount_in as u128, fee_rate);

    // 3) Constant-product formula: amount_out = y - (k / (x + effective_in))
    // k = x * y
    let x = vault_a_balance;
    let y = vault_b_balance;
    let k = x.checked_mul(y).ok_or(ErrorCode::MathOverflow)?;
    let new_x = x.checked_add(effective_in).ok_or(ErrorCode::MathOverflow)?;
    // new_y = k / new_x  (integer division)
    let new_y = k.checked_div(new_x).ok_or(ErrorCode::MathOverflow)?;
    let amount_out = y.checked_sub(new_y).ok_or(ErrorCode::MathOverflow)?;

    // 4) Sanity: enforce min_out
    let amount_out_u64 = amount_out.try_into().map_err(|_| ErrorCode::MathOverflow)?;
    if amount_out_u64 < min_out {
        return Err(error!(ErrorCode::SlippageExceeded));
    }

    // 5) Transfer tokens: user -> pool_vault_a (amount_in), pool_vault_b -> user (amount_out)
    // And handle fee transfer to fee destination (not shown — you would transfer fee_amount to fee receiver)
    // We'll call token transfer helper functions (not fully implemented here)
    token_transfer(&ctx.accounts.user_source, pool_vault_a_info, amount_in)?;
    token_transfer(pool_vault_b_info, &ctx.accounts.user_destination, amount_out_u64)?;

    // 6) Update pool accounting if necessary
    // (total_liquidity might not change for a swap; but if your design tracks cumulative fees, you can record them)
    Ok(())
}

// Helpers (stubs) — in real code you must parse the token account and call the token program CPI
fn get_token_balance(_acct: &AccountInfo) -> Result<u64> {
    // parse token account data and return .amount
    // stub for explanation; implement using spl_token::state::Account::unpack
    Ok(1_000_000u64)
}

fn token_transfer(_from: &AccountInfo, _to: &AccountInfo, _amount: u64) -> Result<()> {
    // perform CPI to token program for transfer
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Slippage exceeded")]
    SlippageExceeded,
}

use crate::fees::apply_fee;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

pub fn execute_swap(
    ctx: Context<crate::context::ExecuteSwap>,
    amount_in: u64,
    min_out: u64,
) -> Result<()> {
    // 1) Read pool vault balances from TokenAccount
    let vault_a_balance = ctx.accounts.pool_vault_a.amount as u128;
    let vault_b_balance = ctx.accounts.pool_vault_b.amount as u128;

    // 2) Apply fee to input amount
    let fee_rate = ctx.accounts.pool_state.fee_rate_bps;
    let (effective_in, fee_amount) = apply_fee(amount_in as u128, fee_rate);

    // 3) Constant-product formula
    let x = vault_a_balance;
    let y = vault_b_balance;
    let k = x.checked_mul(y).ok_or(ErrorCode::MathOverflow)?;
    let new_x = x.checked_add(effective_in).ok_or(ErrorCode::MathOverflow)?;
    let new_y = k.checked_div(new_x).ok_or(ErrorCode::MathOverflow)?;
    let amount_out = y.checked_sub(new_y).ok_or(ErrorCode::MathOverflow)?;

    // 4) Check slippage
    let amount_out_u64 = amount_out.try_into().map_err(|_| ErrorCode::MathOverflow)?;
    if amount_out_u64 < min_out {
        return Err(error!(ErrorCode::SlippageExceeded));
    }

    // 5) Transfer tokens using proper CPI
    // Transfer from user to pool vault A
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_source.to_account_info(),
        to: ctx.accounts.pool_vault_a.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount_in)?;

    // Transfer from pool vault B to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.pool_vault_b.to_account_info(),
        to: ctx.accounts.user_destination.to_account_info(),
        authority: ctx.accounts.pool_authority.to_account_info(),
    };
    let seeds = &[
        &ctx.accounts.pool_state.key().to_bytes()[..32],
        &[ctx.accounts.pool_state.authority_bump],
    ];
    let signer_seeds = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, amount_out_u64)?;

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Slippage exceeded")]
    SlippageExceeded,
}

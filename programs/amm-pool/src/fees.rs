pub fn apply_fee(amount: u128, fee_bps: u64) -> (u128, u128) {
    // fee_bps: basis points, e.g., 30 = 0.30%
    let fee = amount
        .checked_mul(fee_bps as u128)
        .unwrap()
        .checked_div(10_000u128)
        .unwrap();
    let after = amount.checked_sub(fee).unwrap();
    (after, fee)
}

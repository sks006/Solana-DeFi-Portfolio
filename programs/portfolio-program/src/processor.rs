pub fn update(ctx: Context<UpdatePosition>, asset: Pubkey, delta: i64) -> Result<()> {
    let position = &mut ctx.accounts.position;
    position.amount += delta;
    Ok(())
}

pub fn record(ctx: Context<RecordTrade>, trade: TradeData) -> Result<()> {
    // Save trade history, update PnL
    Ok(())
}

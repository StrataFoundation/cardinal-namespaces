use {crate::state::*, anchor_lang::prelude::*};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateNamespaceIx {
    pub update_authority: Pubkey,
    pub rent_authority: Pubkey,
    pub approve_authority: Option<Pubkey>,
    // payment
    pub payment_amount_daily: u64,
    pub payment_mint: Pubkey,
    // validators
    pub min_rental_seconds: i64,
    pub max_rental_seconds: Option<i64>,
    pub transferable_entries: bool,
}

#[derive(Accounts)]
pub struct UpdateNamepsace<'info> {
    #[account(
        mut,
        seeds = [NAMESPACE_PREFIX.as_bytes(), namespace.name.as_ref()],
        bump = namespace.bump,
    )]
    pub namespace: Account<'info, Namespace>,
    #[account(constraint = namespace.update_authority == update_authority.key())]
    pub update_authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateNamepsace>, ix: UpdateNamespaceIx) -> ProgramResult {
    let namespace = &mut ctx.accounts.namespace;
    namespace.update_authority = ix.update_authority;
    namespace.rent_authority = ix.rent_authority;
    namespace.approve_authority = ix.approve_authority;
    namespace.payment_amount_daily = ix.payment_amount_daily;
    namespace.payment_mint = ix.payment_mint;
    namespace.min_rental_seconds = ix.min_rental_seconds;
    namespace.max_rental_seconds = ix.max_rental_seconds;
    namespace.transferable_entries = ix.transferable_entries;
    Ok(())
}

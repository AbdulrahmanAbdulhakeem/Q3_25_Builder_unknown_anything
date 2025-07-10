use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker:Signer<'info>,
}
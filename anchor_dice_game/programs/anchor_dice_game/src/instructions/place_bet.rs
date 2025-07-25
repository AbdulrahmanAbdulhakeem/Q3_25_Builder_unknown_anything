use anchor_lang::prelude::*;

use crate::Bet;

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub house:UncheckedAccount<'info>,
    #[account(
        init,
        payer = player,
        space = 8 + Bet::INIT_SPACE,
        seeds = [b"bet" , player.key().as_ref()],
        bump
    )]
    pub BetAccount: Account<'info, Bet>,
    #[account(
        mut,
         seeds = [b"vault" , house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

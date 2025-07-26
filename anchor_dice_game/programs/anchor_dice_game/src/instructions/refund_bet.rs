use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::Bet;

#[derive(Accounts)]
pub struct RefundBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    pub player: UncheckedAccount<'info>,
    #[account(
         mut,
        seeds = [b"vault" , house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"bet" , player.key().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}

impl<'info> RefundBet<'info> {
    pub fn refund_bet(&self,bumps:&RefundBetBumps) -> Result<()> {
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };

        let seeds = [b"vault", &self.house.key.to_bytes()[..], &[bumps.vault]];
        let signer_seeds = &[&seeds[..]][..];

        let ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);

        transfer(ctx,self.bet.amount)
    }
}

use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{error::ChainPostError, PostAccount, UserAccountState};

#[derive(Accounts)]
pub struct TiPPost<'info>{
    #[account(mut)]
    pub tipper:Signer<'info>,
    pub content_creator:SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"user" , tipper.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccountState>,
     #[account(
        mut,
        seeds = [b"post",post_account.author.to_bytes().as_ref(),post_account.seed.to_le_bytes().as_ref()],
        bump
    )]
    pub post_account: Account<'info, PostAccount>,
    pub system_program:Program<'info,System>
}

impl<'info>TiPPost<'info>{
    pub fn tip_sol(&mut self,amount:u64) -> Result<()> {
        require!(self.content_creator.key() == self.post_account.author.key(),ChainPostError::InvalidCreator);

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{ from: self.tipper.to_account_info(), to: self.content_creator.to_account_info()};

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        self.post_account.tip_total += amount;
        self.user_account.total_tip += amount;

        Ok(())
    }
}
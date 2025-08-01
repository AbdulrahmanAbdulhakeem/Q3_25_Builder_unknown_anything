use anchor_lang::prelude::*;

use crate::Dao;

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + Dao::INIT_SPACE,
        seeds = [b"dao",creator.key().as_ref(),name.as_bytes()],
        bump
    )]
    pub dao_account: Account<'info, Dao>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init_dao(&mut self, name: String,bumps:&InitializeBumps) -> Result<()> {
        self.dao_account.set_inner(Dao {
            name,
            authority: self.creator.key(),
            bump: bumps.dao_account,
            proposal_count: 0,
        });
        
        Ok(())
    }
}

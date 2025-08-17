use anchor_lang::prelude::*;

use crate::{CommentAccount, PostAccount};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct CommentOnPost<'info> {
    #[account(mut)]
    pub commenter: Signer<'info>,
     #[account(mut)]
    pub author:SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = commenter,
        space = 8 + CommentAccount::INIT_SPACE,
        seeds = [b"comment",post_account.key().as_ref(),seed.to_le_bytes().as_ref(),commenter.key().as_ref()],
        bump
    )]
    pub comment_account: Account<'info, CommentAccount>,
    #[account(
        mut,
        seeds = [b"post",author.key.as_ref(),post_account.seed.to_le_bytes().as_ref()],
        bump = post_account.bump
    )]
    pub post_account: Account<'info, PostAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> CommentOnPost<'info> {
    pub fn comment(&mut self,seed:u64,title:String,comment:String,bumps:&CommentOnPostBumps) -> Result<()> {
        let clock = Clock::get()?;
        

        self.comment_account.set_inner(CommentAccount {
            owner: self.commenter.key(),
            title,
            comment,
            post_key: self.post_account.key(),
            post_owner: self.post_account.author.key(),
            timestamp: clock.unix_timestamp,
            bump: bumps.comment_account,
            seed,
        });

        Ok(())
    }
}

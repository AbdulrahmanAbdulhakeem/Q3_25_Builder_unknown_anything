use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::{Dao, Proposal, Vote};

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    pub dao_account: Account<'info, Dao>,
    pub proposal_account: Account<'info, Proposal>,
    #[account(
        init,
        payer = voter,
        space = 8 + Vote::INIT_SPACE,
        seeds = [b"vote",voter.key().as_ref(),proposal_account.key().as_ref()],
        bump
    )]
    pub vote_account: Account<'info, Vote>,
    #[account(
        token::authority = voter
    )]
    pub creator_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info>CastVote<'info> {
    pub fn cast_vote(&mut self, vote_type: u8,bumps:&CastVoteBumps) -> Result<()> {
        let voting_credits = (self.creator_token_account.amount as f64).sqrt() as u64;

        self.vote_account.set_inner(Vote {
            authority:self.voter.key(),
            vote_type,
            vote_credit: voting_credits,
            bump: bumps.vote_account,
        });

        Ok(())
    }
}

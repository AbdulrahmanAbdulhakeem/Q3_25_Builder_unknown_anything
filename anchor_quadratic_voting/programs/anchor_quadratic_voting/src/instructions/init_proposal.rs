use anchor_lang::prelude::*;

use crate::{Dao, Proposal};

#[derive(Accounts)]
pub struct InitProposal<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub dao_account: Account<'info, Dao>,
    #[account(
        init,
        payer = creator,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal",dao_account.key().as_ref(),dao_account.proposal_count.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal_account: Account<'info, Proposal>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitProposal<'info> {
    pub fn init_proposal(&mut self, metadata: String, bumps: &InitProposalBumps) -> Result<()> {
        self.proposal_account.set_inner(Proposal {
            authority:self.creator.key(),
            metadata,
            yes_vote_count: 0,
            no_vote_count: 0,
            bump: bumps.proposal_account,
        });
        
        Ok(())
    }
}

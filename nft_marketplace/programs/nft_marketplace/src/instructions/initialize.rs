use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{error::MarketplaceError, Marketplace};

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace" , name.as_bytes()],
        bump,
        space = 8 + Marketplace::INIT_SPACE
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"treasury" , marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program:Interface<'info,TokenInterface>
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, fees: u32, name: String,bumps:&InitializeBumps) -> Result<()> {
        require!(name.len() >= 1 && name.len() <= 32,MarketplaceError::NameTooLong);

        self.marketplace.set_inner(Marketplace {
            bump: bumps.marketplace,
            admin: self.admin.key(),
            reward_bump: bumps.reward_mint,
            treasury_bump: bumps.treasury,
            fees,
            name,
        });
        
        Ok(())
    }
}

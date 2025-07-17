use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::StakeConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        seeds = [b"config"],
        bump,
        space = 8 + StakeConfig::INIT_SPACE
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init,
        payer = payer,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init(
        &mut self,
        points_per_stake: u8,
        freeze_period: u32,
        max_stake: u8,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StakeConfig {
            point_per_stake:points_per_stake,
            reward_bump: bumps.reward_mint,
            bump:bumps.config,
            freeze_period,
            max_stake:max_stake,
        });

        Ok(())
    }
}

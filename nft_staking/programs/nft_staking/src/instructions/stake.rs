use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::error::*;
use crate::{StakeAccountState, StakeConfig, UserAccountState};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub nft_mint_ata: Account<'info, TokenAccount>,
    pub collection_mint: Account<'info, Mint>,
    #[account(
        seeds = [b"metadata", nft_mint.key().as_ref(),metadata_program.key().as_ref()],
        bump,
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [b"metadata",nft_mint.key().as_ref(),metadata_program.key().as_ref(),b"edition"],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub edition: Account<'info, MasterEditionAccount>,
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"user" , user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccountState>,
    #[account(
        init,
        payer = user,
        space = 8 + StakeAccountState::INIT_SPACE,
        seeds = [b"stake_account" , nft_mint.key().as_ref(),config.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccountState>,

    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        require!(
            self.user_account.amount_staked < self.config.max_stake,
            StakeError::MaxStakeReeached
        );

        self.user_account.amount_staked += 1;

        let clock = Clock::get()?;

        self.stake_account.set_inner(StakeAccountState {
            nft_mint: self.nft_mint.key(),
            owner: self.user.key(),
            bump: bumps.stake_account,
            staked_at: clock.unix_timestamp,
        });

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Approve {
            to: self.nft_mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?;

        let seeds = &[
            b"stake_account",
            self.nft_mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let metadata_program = &self.metadata_program.to_account_info();
        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.nft_mint_ata.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.nft_mint.to_account_info();

        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                token_program,
                edition,
                mint,
            },
        )
        .invoke_signed(signer_seeds)?;
        Ok(())
    }
}

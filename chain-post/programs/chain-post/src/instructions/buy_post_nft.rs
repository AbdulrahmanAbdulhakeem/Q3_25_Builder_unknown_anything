use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::token_interface::{Mint, TokenInterface};
use mpl_core::{instructions::CreateV1CpiBuilder, types::DataState};

use crate::{error::ChainPostError, PostAccount, UserAccountState};

#[derive(Accounts)]
pub struct BuyPostNft<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub author: SystemAccount<'info>,
    /// CHECK: This is the mint account of the asset to be minted
    #[account(mut)]
    pub nft_mint: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user",buyer.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccountState>,
    #[account(
        mut,
        seeds = [b"post",author.key.as_ref(),post_account.seed.to_le_bytes().as_ref()],
        bump = post_account.bump
    )]
    pub post_account: Account<'info, PostAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core: UncheckedAccount<'info>,
}

impl<'info> BuyPostNft<'info> {
    pub fn transfer_sol(&mut self, amount: u64) -> Result<()> {
        require!(
            self.author.key() == self.post_account.author.key(),
            ChainPostError::InvalidCreator
        );

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.author.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn mint_nft_to_buyer(&mut self, name: String, uri: String) -> Result<()> {
        CreateV1CpiBuilder::new(&self.mpl_core.to_account_info())
            .asset(&self.nft_mint.to_account_info())
            .authority(Some(&self.author.to_account_info()))
            .payer(&self.buyer.to_account_info())
            .owner(Some(&self.buyer.to_account_info()))
            .update_authority(Some(&self.author.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(name)
            .uri(uri)
            .data_state(DataState::AccountState).invoke()?;

        self.post_account.nft_sold += 1;
        Ok(())
    }
}

use anchor_lang::prelude::*;
use mpl_bubblegum::instructions::BurnV2CpiBuilder;
use spl_account_compression::program::SplAccountCompression;

use crate::PostAccount;

#[derive(Accounts)]
pub struct DeletePost<'info> {
    #[account(mut)]
    pub creator_or_admin: Signer<'info>,
    #[account(
        mut,
        close = creator_or_admin,
        seeds = [b"post",post_account.author.key().to_bytes().as_ref(),post_account.seed.to_le_bytes().as_ref()],
        bump
    )]
    pub post_account: Account<'info, PostAccount>,
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key(),
    )]
    ///CHECK:
    pub tree_config: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account is neither written to nor read from.
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub log_wrapper: UncheckedAccount<'info>,
    pub compression_program: Program<'info, SplAccountCompression>,
    /// CHECK: This account is neither written to nor read from.
    pub bubblegum_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> DeletePost<'info> {
    pub fn burn_nft(
        &mut self,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        BurnV2CpiBuilder::new(&self.bubblegum_program.to_account_info())
            .tree_config(&self.tree_config.to_account_info())
            .payer(&self.creator_or_admin)
            .leaf_owner(&self.creator_or_admin)
            .leaf_delegate(Some(&self.creator_or_admin))
            .merkle_tree(&self.merkle_tree.to_account_info())
            .log_wrapper(&self.log_wrapper.to_account_info())
            .compression_program(&self.compression_program.to_account_info())
            .system_program(&self.system_program.to_account_info()).root(root).data_hash(data_hash).creator_hash(creator_hash).nonce(nonce).index(index);
        Ok(())
    }
}

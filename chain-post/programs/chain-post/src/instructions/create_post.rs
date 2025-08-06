use crate::metaplex_adapter::{MetadataArgsV2Local};
use crate::{MplBubblegum, PlatformConfig, PostAccount, UserAccountState};
use anchor_lang::prelude::*;
use mpl_bubblegum;
use mpl_bubblegum::{instructions::MintV2CpiBuilder, types::MetadataArgsV2};
use spl_account_compression::{program::SplAccountCompression, Noop};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct CreatePost<'info> {
    #[account(mut)]
    pub content_creator: Signer<'info>,
    pub admin: SystemAccount<'info>,
    #[account(mut)]
    ///CHECK:This is safe
    pub merkle_tree: UncheckedAccount<'info>,
    #[account(
        init,
        payer = content_creator,
        space = 8 + PostAccount::INIT_SPACE,
        seeds = [b"post",content_creator.key().to_bytes().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub post_account: Account<'info, PostAccount>,
    #[account(
        mut,
        seeds = [b"user" , content_creator.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccountState>,
    #[account(
        mut,
        seeds = [b"platformConfig", platform_config.authority.key().to_bytes().as_ref()],
        bump = platform_config.bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key(),
    )]
    ///CHECK:
    pub tree_config: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> CreatePost<'info> {
    pub fn create_cnft_post(
        &mut self,
        metadata: MetadataArgsV2Local,
        seed: u64,
        bumps: &CreatePostBumps,
        content: String,
    ) -> Result<()> {
        let main_metadata = MetadataArgsV2 {
            name: metadata.name,
            symbol: metadata.symbol,
            uri: metadata.uri,
            seller_fee_basis_points: metadata.seller_fee_basis_points,
            primary_sale_happened: metadata.primary_sale_happened,
            is_mutable: metadata.is_mutable,
            token_standard:None,
            creators:vec![],
            collection:None,
        };
        
        MintV2CpiBuilder::new(&self.bubblegum_program.to_account_info())
            .tree_config(&self.tree_config.to_account_info())
            .payer(&self.content_creator.to_account_info())
            .tree_creator_or_delegate(Some(&self.admin.to_account_info()))
            .leaf_owner(&self.content_creator.to_account_info())
            .leaf_delegate(Some(&self.admin.to_account_info()))
            .merkle_tree(&self.merkle_tree.to_account_info())
            .log_wrapper(&self.log_wrapper.to_account_info())
            .compression_program(&self.compression_program.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .mpl_core_program(&self.mpl_core_program.to_account_info())
            .metadata(main_metadata)
            .invoke()
            .unwrap();

        let clock = Clock::get()?;

        self.post_account.set_inner(PostAccount {
            merkle_tree: self.merkle_tree.key(),
            bump: bumps.post_account,
            author: self.content_creator.key(),
            timestamp: clock.unix_timestamp,
            tip_total: 0,
            content,
            seed,
            nft_sold: 0,
        });

        self.user_account.post_created += 1;

        Ok(())
    }
}

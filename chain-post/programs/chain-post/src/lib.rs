#![allow(deprecated, unexpected_cfgs)]
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("5xypnA1UCCrXo2kXvcAksKnsRf1oigFhasGQGgfZntqc");

#[derive(Clone)]
pub struct MplBubblegum;

impl anchor_lang::Id for MplBubblegum {
    fn id() -> Pubkey {
        mpl_bubblegum::ID
    }
}

// ctx: Context<'_, '_, '_, 'info, Mint<'info>>,
#[program]
pub mod chain_post {

    // use crate::metaplex_adapter::MetadataArgsLocal;

    // use mpl_bubblegum::types::MetadataArgs;

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        max_depth: u32,
        max_buffer_size: u32,
    ) -> Result<()> {
        ctx.accounts
            .create_merkle_tree(max_depth, max_buffer_size)?;
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUserConfig>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        seed: u64,
        content: String,
    ) -> Result<()> {
        ctx.accounts
            .create_cnft_post(seed, &ctx.bumps, content)?;
        Ok(())
    }

    //For now only sol is supported
    pub fn tip_post(ctx: Context<TiPPost>,seed: u64, amount: u64) -> Result<()> {
        ctx.accounts.tip_sol(amount)?;
        Ok(())
    }

    pub fn comment_on_post(
        ctx: Context<CommentOnPost>,
        seed: u64,
        title: String,
        comment: String,
    ) -> Result<()> {
        ctx.accounts.comment(seed, title, comment, &ctx.bumps)?;
        Ok(())
    }

    pub fn delete_post(
        ctx: Context<DeletePost>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        ctx.accounts
            .burn_nft(root, data_hash, creator_hash, nonce, index)?;
        msg!("Post has been Deleted");
        msg!("CNFT burned");
        Ok(())
    }

    pub fn buy_post_nft(
        ctx: Context<BuyPostNft>,
        amount: u64,
        name: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.transfer_sol(amount)?;
        ctx.accounts.mint_nft_to_buyer(name, uri)?;
        Ok(())
    }
}

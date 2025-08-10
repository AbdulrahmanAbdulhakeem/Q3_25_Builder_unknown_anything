use anchor_lang::prelude::*;
// use mpl_account_compression::{program::MplAccountCompression,Noop};
use mpl_bubblegum::instructions::CreateTreeConfigCpiBuilder;
use spl_account_compression::{program::SplAccountCompression, Noop};



use crate::{MplBubblegum, PlatformConfig};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    ///CHECK:
    #[account(mut,signer)]
    pub merkle_tree: UncheckedAccount<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + PlatformConfig::INIT_SPACE,
        seeds = [b"platformConfig", admin.key().as_ref()],
        bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    ///CHECK:
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key(),
    )]
    pub tree_config: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
}

impl<'info> Initialize<'info> {
    pub fn create_merkle_tree(&mut self, max_depth: u32, max_buffer_size: u32) -> Result<()> {
        CreateTreeConfigCpiBuilder::new(&self.bubblegum_program.to_account_info())
            .merkle_tree(&self.merkle_tree.to_account_info())
            .tree_config(&self.tree_config.to_account_info())
            .payer(&self.admin.to_account_info())
            .log_wrapper(&self.log_wrapper.to_account_info())
            .compression_program(&self.compression_program.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .tree_creator(&self.admin.to_account_info())
            .max_depth(max_depth)
            .max_buffer_size(max_buffer_size)
            .public(false)
            .invoke()?;
        Ok(())
    }

    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.platform_config.set_inner(PlatformConfig {
            merkle_tree: self.merkle_tree.key(),
            bump: bumps.platform_config,
            authority: self.admin.key(),
        });

        Ok(())
    }
}

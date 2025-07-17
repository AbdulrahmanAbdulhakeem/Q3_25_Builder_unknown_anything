#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5Zqk9MMXeweczXeqDsCN3M96QjqtJLLSk8KrDCMhdYng");

#[program]
pub mod nft_staking {
    use crate::instruction::Stake;

    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>,points_per_stake:u8,freeze_period:u32,max_state:u8) -> Result<()> {
        ctx.accounts.init(points_per_stake, freeze_period, max_state, &ctx.bumps)
    }

    pub fn initialize_user(ctx:Context<InitializeUserConfig>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }

    pub fn stake(ctx:Context<Stake>) -> Result<()>{
        Ok(())
    }
}

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
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}

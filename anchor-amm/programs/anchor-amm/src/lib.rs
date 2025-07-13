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

declare_id!("6jbiwW5Hahf2Ewj7KcPYFLRqhYXeChvXxg1tYSz4foeq");

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,seed:u64,fees:u16,authority:Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed, fees, authority, &ctx.bumps)
    }
}

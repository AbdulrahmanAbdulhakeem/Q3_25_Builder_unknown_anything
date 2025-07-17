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

declare_id!("5mNJ9jtaYfLRBCg5gwLc8WSsGmDQJ8buwWZ65whMRNHV");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fees:u32,name:String) -> Result<()> {
        ctx.accounts.initialize(fees, name, &ctx.bumps)
    }

    pub fn list(ctx: Context<List>,price:u64) -> Result<()>{
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }
}


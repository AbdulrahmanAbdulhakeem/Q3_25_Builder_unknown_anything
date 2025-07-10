#![allow(unexpected_cfgs,deprecated)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Ek8ecZ1dZrx2Psc4viGJaBKF1MLYdU8vbnbZE6sus1Yk");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>,seed:u64,receive:u64,deposit:u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }
}

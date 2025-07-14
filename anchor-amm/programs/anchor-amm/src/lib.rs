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

    pub fn withdraw(ctx: Context<Withdraw>,lp_amount:u64,min_x:u64,min_y:u64) -> Result<()>{
        ctx.accounts.withdraw(lp_amount, min_x, min_y)
    }
    pub fn deposit(ctx: Context<Deposit>,max_y:u64,max_x:u64,lp_amount:u64) -> Result<()>{
        ctx.accounts.deposit(max_y, max_x, lp_amount)
    }
    pub fn swap(ctx: Context<Swap>,is_x:bool,amount:u64,min:u64) -> Result<()>{
        ctx.accounts.swap(is_x, amount, min)
    }
}

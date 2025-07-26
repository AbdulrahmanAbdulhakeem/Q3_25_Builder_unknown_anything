pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("B43tD6CujgbqVKSsxsoaU8r56wdXC5BNyYe6WyVHw8kg");

#[program]
pub mod anchor_dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,amount:u64) -> Result<()> {
        ctx .accounts.init(amount)
    }

    pub fn place_bet(ctx:Context<PlaceBet>,seed:u128,amount:u64,roll:u8) -> Result<()>{
        ctx.accounts.create_bet(seed, amount, &ctx.bumps, roll)?;
        ctx.accounts.deposit(amount)
    }

    pub fn resolve_bet(ctx:Context<ResolveBet>,sig:Vec<u8>) -> Result<()>{
        ctx.accounts.verify_edd25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&sig, &ctx.bumps)
    }

    pub fn refund_bet(ctx:Context<RefundBet>) -> Result<()>{
        ctx.accounts.refund_bet(&ctx.bumps)
    }
}

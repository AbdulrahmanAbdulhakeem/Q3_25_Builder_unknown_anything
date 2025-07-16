use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccountState{
    pub nft_mint:u32,
    pub owner:Pubkey,
    pub bump:u8,
    pub staked_at:u8
}
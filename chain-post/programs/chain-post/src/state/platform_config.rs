use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlatformConfig{
    pub merkle_tree:Pubkey,
    pub bump:u8,
    pub authority:Pubkey
}
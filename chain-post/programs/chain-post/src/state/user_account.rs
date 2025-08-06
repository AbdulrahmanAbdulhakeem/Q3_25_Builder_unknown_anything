use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccountState{
    pub bump:u8,
    pub post_created:u16,
    pub total_tip:u64,
    pub owner:Pubkey
}
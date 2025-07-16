use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccountState {
    pub bump:u8,
    pub amount_staked:u8,
    pub points:u32
}
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig{
    pub point_per_stake:u8,
    pub reward_bump:u8,
    pub bump:u8,
    pub freeze_period:u32,
    pub max_stake:u8
}
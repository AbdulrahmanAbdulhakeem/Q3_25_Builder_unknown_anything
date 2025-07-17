use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace{
    pub bump:u8,
    pub admin:Pubkey,
    pub reward_bump:u8,
    pub treasury_bump:u8,
    pub fees:u32,
    #[max_len(32)]
    pub name:String
}
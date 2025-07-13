use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seed:u64,
    pub locked:bool,
    pub mint_x:Pubkey,
    pub mint_y:Pubkey,
    pub authority:Option<Pubkey>,
    pub fees:u16,
    pub lp_bump:u8,
    pub config_bump:u8
}

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PostAccount{
    pub merkle_tree:Pubkey,
    pub bump:u8,
    pub author:Pubkey,
    pub timestamp:i64,
    pub tip_total:u64,
    #[max_len(5000)]
    pub content:String,
    pub seed:u64,
    pub nft_sold:u64
}
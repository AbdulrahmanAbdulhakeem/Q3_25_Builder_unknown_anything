use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CommentAccount{
    pub owner:Pubkey,
    #[max_len(100)]
    pub title:String,
    #[max_len(1000)]
    pub comment:String,
    pub(crate) post_key:Pubkey,
    pub(crate) post_owner:Pubkey,
    pub timestamp:i64,
    pub bump:u8,
    pub seed:u64
}
use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Dao{
    #[max_len(500)]
    pub name:String,
    pub authority:Pubkey,
    pub bump:u8,
    pub proposal_count:u64
}

#[account]
#[derive(InitSpace)]
pub struct Vote{
    pub authority:Pubkey,
    pub vote_type:u8,
    pub vote_credit:u64,
    pub bump:u8
}

#[account]
#[derive(InitSpace)]
pub struct Proposal{
    pub authority:Pubkey,
    #[max_len(500)]
    pub metadata:String,
    pub yes_vote_count:u64,
    pub no_vote_count:u64,
    pub bump:u8
}
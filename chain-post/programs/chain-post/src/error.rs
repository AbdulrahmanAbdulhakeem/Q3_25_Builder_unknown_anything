use anchor_lang::prelude::*;

#[error_code]
pub enum ChainPostError {
    #[msg("This is not the post owner")]
    InvalidCreator,
}

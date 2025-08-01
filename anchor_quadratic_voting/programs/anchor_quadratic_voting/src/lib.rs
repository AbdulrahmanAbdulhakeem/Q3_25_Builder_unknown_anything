#![allow(deprecated,unexpected_cfgs)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6Js9GNjzsFW1oupmPqnUEUPtiMswK9xUatoUktfKKiTa");

#[program]
pub mod anchor_quadratic_voting {
    use super::*;

    pub fn intialize_dao(ctx:Context<Initialize>,name:String) -> Result<()>{
        ctx.accounts.init_dao(name, &ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_proposal(ctx:Context<InitProposal>,metadata:String) -> Result<()>{
        ctx.accounts.init_proposal(metadata, &ctx.bumps)?;
        Ok(())
    }

    pub fn vote(ctx:Context<CastVote>,vote_type:u8) -> Result<()>{
        ctx.accounts.cast_vote(vote_type, &ctx.bumps)?;
        Ok(())
    }
   
}

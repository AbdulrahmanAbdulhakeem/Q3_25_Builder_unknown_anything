#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("34AuAC3qLVb5uBSgfTSPea9Q2zcuWf2SQFMjQY2sm3Lg");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        payer = user,
        space = VaultState::INIT_SPACE,
        seeds = [b"vault" , user.key().as_ref()],
        bump
    )]
    pub vault_state:Account<'info,VaultState>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault:SystemAccount<'info>,
    pub system_program:Program<'info,System>
}

impl<'info>Initialize<'info> {
    pub fn initialize (&mut self, amount:u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from:self.user.to_account_info(),
            to:self.vault.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }
}

#[account]
pub struct VaultState {
    pub vault_bump:u8,
    pub state_bump:u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1*2;
}
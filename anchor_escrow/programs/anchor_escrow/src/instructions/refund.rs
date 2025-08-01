use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface,TransferChecked,transfer_checked,close_account,CloseAccount}};

use crate::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker:Signer<'info>,
    pub mint_a:InterfaceAccount<'info,Mint>,
    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = mint_a,
        associated_token::authority = maker
    )]
    pub maker_ata_a:InterfaceAccount<'info,TokenAccount>,
    #[account(
        seeds = [b"escrow" , maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow:Account<'info,Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,

    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>
}

impl<'info>Refund<'info>{
    pub fn refund_and_close_vault(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let transfer_accounts = TransferChecked {
            from:self.vault.to_account_info(),
            to:self.maker_ata_a.to_account_info(),
            authority:self.escrow.to_account_info(),
            mint:self.mint_a.to_account_info()
        };

        let signer_seeds:[&[&[u8]];1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump]
        ]];
        

        let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), transfer_accounts, &signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        let close_accounts = CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.maker.to_account_info(),
            authority:self.escrow.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), close_accounts, &signer_seeds);

        close_account(cpi_ctx)
    }
}
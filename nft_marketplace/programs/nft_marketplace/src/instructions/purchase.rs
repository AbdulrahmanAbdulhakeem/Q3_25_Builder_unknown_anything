use anchor_lang::{prelude::*, system_program::{Transfer,transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface,transfer_checked,TransferChecked,CloseAccount,close_account}};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata:InterfaceAccount<'info,TokenAccount>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        seeds = [b"listing",maker_mint.key().as_ref(),marketplace.key().as_ref()],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        seeds = [b"treasury" , marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn send_sol_to_maker(&mut self) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, self.listing.price)?;

        let fee = self
            .listing
            .price
            .checked_mul(self.marketplace.fees as u64)
            .unwrap()
            .checked_div(10000)
            .unwrap();

        let cpi_account = Transfer{
            from:self.taker.to_account_info(),
            to:self.treasury.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_account);

        transfer(cpi_ctx, fee)
    }

    pub fn transfer_nft_to_taker(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked{
            from:self.vault.to_account_info(),
            mint:self.maker_mint.to_account_info(),
            to:self.taker_ata.to_account_info(),
            authority:self.listing.to_account_info()
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"listing",
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, &signer_seeds);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)
    }

    pub fn close_vault(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_account = CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.maker.to_account_info(),
            authority:self.listing.to_account_info()
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"listing",
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, &signer_seeds);

        close_account(cpi_ctx)
    }
}

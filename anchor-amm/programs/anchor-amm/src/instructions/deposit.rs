use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};
use constant_product_curve::ConstantProduct;

use crate::error::AmmError;
use crate::Config;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub lp_provider: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", mint_x.key().to_bytes().as_ref(),mint_y.key().to_bytes().as_ref(),config.seed.to_be_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        seeds = [b"lp",config.key().as_ref()],
        bump = config.lp_bump,
        mint::decimals = 6,
        mint::authority = config
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = lp_provider,
        associated_token::token_program = token_program
    )]
    pub lp_provider_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = lp_provider,
        associated_token::token_program = token_program
    )]
    pub lp_provider_ata_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_lp,
        associated_token::authority = mint_lp,
        associated_token::token_program = token_program
    )]
    pub lp_provider_ata_lp: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, max_y: u64, max_x: u64, lp_amount: u64) -> Result<()> {
        // lp_amount: amount of LP Tokens that the user wants to claim
        // max_x: the max amount of x they are willing to deposit
        // max_y: the max amount of y they are willing to deposit

        require!(lp_amount > 0, AmmError::InvalidAmount);
        require!(!self.config.locked, AmmError::AMMLocked);

        let (x, y) = match self.mint_lp.supply == 0
            && self.vault_x.amount == 0
            && self.vault_y.amount == 0
        {
            true => (max_x, max_y),
            false => {
                let amount = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    lp_amount,
                    6,
                ).unwrap();
                (amount.x,amount.y)
            }
        };

        require!(max_x >= x,AmmError::InsufficientTokenX);
        require!(max_y >= y,AmmError::InsufficientTokenY);

        self.deposit_tokens(true, x)?;
        self.deposit_tokens(false, y)?;
        self.mint_lp_token(lp_amount)?;
        Ok(())
    }

    fn deposit_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts, mint_decimals) = match is_x {
            true => (
                TransferChecked {
                    from: self.lp_provider_ata_x.to_account_info(),
                    mint: self.mint_x.to_account_info(),
                    to: self.vault_x.to_account_info(),
                    authority: self.lp_provider.to_account_info(),
                },
                self.mint_x.decimals,
            ),
            false => (
                TransferChecked {
                    from: self.lp_provider_ata_y.to_account_info(),
                    mint: self.mint_y.to_account_info(),
                    to: self.vault_y.to_account_info(),
                    authority: self.lp_provider.to_account_info(),
                },
                self.mint_y.decimals,
            ),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, mint_decimals)
    }

    fn mint_lp_token(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.lp_provider_ata_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let mint_x = self.mint_x.key().to_bytes();
        let mint_y = self.mint_x.key().to_bytes();
        let seed = self.config.seed.to_be_bytes();

        let seeds = [b"config", mint_x.as_ref(), mint_y.as_ref(), seed.as_ref()];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount)
    }
}

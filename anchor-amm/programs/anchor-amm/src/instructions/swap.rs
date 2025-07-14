use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::error::*;
use crate::Config;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"config", mint_x.key().to_bytes().as_ref(),mint_y.key().to_bytes().as_ref(),config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
        has_one = mint_x,
        has_one = mint_y
    )]
    pub config: Account<'info, Config>,
    #[account(
        seeds = [b"lp",config.key().as_ref()],
        bump = config.lp_bump,
        mint::decimals = 6,
        mint::authority = config
    )]
    pub mint_lp:InterfaceAccount<'info,Mint>,
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
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_y: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, is_x: bool, amount: u64, min: u64) -> Result<()> {
        require!(!self.config.locked, AmmError::AMMLocked);
        require!(amount > 0, AmmError::InvalidAmount);

        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            self.config.fees,
            None,
        ).unwrap();

        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y
        };

        let res = curve.swap(p,amount, min).map_err(AmmError::from)?;

        require!(res.deposit != 0,AmmError::InvalidAmount);
        require!(res.withdraw != 0,AmmError::InvalidAmount);

        self.transfer_to_vault(is_x, res.deposit)?;
        self.withdraw_from_vault(is_x, res.withdraw)?;
        Ok(())
    }

    fn transfer_to_vault(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts, mint_decimals) = match is_x {
            true => (
                TransferChecked {
                    from: self.user_ata_x.to_account_info(),
                    mint: self.mint_x.to_account_info(),
                    to: self.vault_x.to_account_info(),
                    authority: self.user.to_account_info(),
                },
                self.mint_x.decimals,
            ),
            false => (
                TransferChecked {
                    from: self.user_ata_y.to_account_info(),
                    mint: self.mint_y.to_account_info(),
                    to: self.vault_y.to_account_info(),
                    authority: self.user.to_account_info(),
                },
                self.mint_y.decimals,
            ),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, mint_decimals)
    }

    fn withdraw_from_vault(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts, mint_decimals) = match is_x {
            true => (
                TransferChecked {
                    from: self.vault_y.to_account_info(),
                    mint: self.mint_y.to_account_info(),
                    to: self.user_ata_y.to_account_info(),
                    authority: self.config.to_account_info(),
                },
                self.mint_y.decimals,
            ),
            false => (
                TransferChecked {
                    from: self.vault_x.to_account_info(),
                    mint: self.mint_x.to_account_info(),
                    to: self.user_ata_y.to_account_info(),
                    authority: self.config.to_account_info(),
                },
                self.mint_x.decimals,
            ),
        };

        let mint_x = self.mint_x.key().to_bytes();
        let mint_y = self.mint_x.key().to_bytes();
        let seed = self.config.seed.to_be_bytes();

        let seeds = [b"config", mint_x.as_ref(), mint_y.as_ref(), seed.as_ref()];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, amount, mint_decimals)
    }
}

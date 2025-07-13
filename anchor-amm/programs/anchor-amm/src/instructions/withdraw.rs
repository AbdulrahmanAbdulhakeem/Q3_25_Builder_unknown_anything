use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    burn, transfer_checked, Burn, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use constant_product_curve::{ConstantProduct, XYAmounts};

use crate::error::AmmError;
use crate::state::Config;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub lp_provider: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config",mint_x.key().to_bytes().as_ref(),mint_y.key().to_bytes().as_ref(),config.seed.to_be_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        seeds = [b"lp", config.key().as_ref()],
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
        init_if_needed,
        payer = lp_provider,
        associated_token::mint = mint_lp,
        associated_token::authority = lp_provider,
        associated_token::token_program = token_program
    )]
    pub lp_provider_ata_lp: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, lp_amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        require!(lp_amount > 0, AmmError::InvalidAmount);
        require!(!self.config.locked, AmmError::AMMLocked);

        let xy_amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            lp_amount,
            6,
        ).unwrap();

        require!(min_x <=xy_amounts.x,AmmError::InsufficientTokenX);
        require!(min_y <= xy_amounts.y,AmmError::InsufficientTokenY);

        self.withdraw_tokens(true, xy_amounts.x)?;
        self.withdraw_tokens(false, xy_amounts.y)?;

        self.burn_lp_tokens(lp_amount)
    }

    fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts, mint_decimals) = match is_x {
            true => (
                TransferChecked {
                    from: self.vault_x.to_account_info(),
                    to: self.lp_provider_ata_x.to_account_info(),
                    mint: self.mint_x.to_account_info(),
                    authority: self.config.to_account_info(),
                },
                self.mint_x.decimals,
            ),
            false => (
                TransferChecked {
                    from: self.vault_y.to_account_info(),
                    to: self.lp_provider_ata_y.to_account_info(),
                    mint: self.mint_y.to_account_info(),
                    authority: self.config.to_account_info(),
                },
                self.mint_y.decimals,
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

    fn burn_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Burn {
            mint: self.mint_lp.to_account_info(),
            from: self.lp_provider_ata_lp.to_account_info(),
            authority: self.lp_provider.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        burn(cpi_ctx, amount)
    }
}

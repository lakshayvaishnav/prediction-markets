use anchor_lang::prelude::*;
use anchor_spl::metadata::Metadata;
use anchor_spl::token_interface::{ Mint, TokenInterface };

use crate::{ Market, PlatformConfig };
use crate::constants::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [PLATFORM_CONFIG],
        bump = platform_config.config_bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    #[account(
        init,
        payer = admin,
        space = 1024,
        seeds = [MARKET, platform_config.key().as_ref(), &name.as_bytes()],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = admin,
        seeds = [MINT_YES, market.key().to_bytes().as_ref()],
        bump,
        mint::authority = platform_config,
        mint::decimals = 6
    )]
    pub mint_yes: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = admin,
        seeds = [MINT_NO, market.key().to_bytes().as_ref()],
        bump,
        mint::authority = platform_config,
        mint::decimals = 6
    )]
    pub mint_no: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [MARKET_VAULT,market.key().to_bytes().as_ref()],
        bump
    )]
    pub market_vault_account: SystemAccount<'info>, // Where bettor desposites there wagers

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub token_metadata_program: Program<'info, Metadata>,
}

impl<'info> CreateMarket<'info> {

    pub fn save_market_data (
        &mut self,
        bump : CreateMarketBumps,
        arg : MargetArg,
        metadata_arg : InitTokenArg,
        
    )

}

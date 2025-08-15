use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{ Mint, TokenAccount, TokenInterface }};

use crate::{
    Bettor,
    Market,
    MarketError,
    PlatformConfig,
    Wager,
    BETTOR_PROFILE,
    BETTOR_WALLET,
    MARKET,
    MARKET_VAULT,
    MINT_NO,
    MINT_YES,
    PLATFORM_CONFIG,
    WAGER,
};

#[derive(Accounts)]
pub struct SellShares<'info> {
    #[account(mut)]
    pub bettor: Signer<'info>,

    #[account(
        mut,
        seeds = [BETTOR_PROFILE,bettor.key().to_bytes().as_ref(),platform_config.key().to_bytes().as_ref()],
        bump = bettor_profile.bettor_bump,
        constraint = bettor_profile.bettor_pubkey == bettor.key() @ MarketError::InvalidAccount,
    )]
    pub bettor_profile: Account<'info, Bettor>,

    #[account(
        mut,
        seeds = [BETTOR_WALLET,bettor.key().to_bytes().as_ref(), platform_config.key().to_bytes().as_ref()],
        bump = bettor_profile.bettor_vault_bump
    )]
    pub bettor_wallet_account: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [PLATFORM_CONFIG],
        bump = platform_config.config_bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    #[account(
        mut,
        seeds = [MARKET, platform_config.key().to_bytes().as_ref(),&market.market_name.as_bytes()[..32]],
        bump = market.market_bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [WAGER,market.key().to_bytes().as_ref(),bettor.key().to_bytes().as_ref()],
        bump = wager_account.bet_bump,
    )]
    pub wager_account: Account<'info, Wager>,

    #[account(
        mut,
        seeds = [MINT_YES,market.key().to_bytes().as_ref()],
        bump = market.mint_yes_bump,
        mint::authority = platform_config,
    )]
    pub mint_yes: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [MINT_NO,market.key().to_bytes().as_ref()],
        bump = market.mint_no_bump,
        mint::authority = platform_config,
    )]
    pub mint_no: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::authority = bettor,
        associated_token::mint = mint_yes,
    )]
    pub bettor_yes_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_no,
        associated_token::authority = bettor,
    )]
    pub bettor_no_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [MARKET_VAULT,market.key().to_bytes().as_ref()],
        bump
    )]
    pub market_vault_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

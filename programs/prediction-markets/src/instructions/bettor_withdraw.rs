use anchor_lang::prelude::*;

use crate::{
    Bettor,
    Market,
    MarketError,
    PlatformConfig,
    Wager,
    BETTOR_PROFILE,
    MARKET,
    PLATFORM_CONFIG,
    WAGER,
};

#[derive(Accounts)]
pub struct BettorWithdraw<'info> {
    #[account(mut)]
    pub bettor: Signer<'info>,

    #[account(
        mut,
        seeds = [PLATFORM_CONFIG],
        bump = platform_config.config_bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    #[account(
        mut,
        seeds = [MARKET, platform_config.key().to_bytes().as_ref(), &market.market_name.as_bytes()[..32]],
        bump = market.market_bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [WAGER,market.key().to_bytes().as_ref(), bettor.key().to_bytes().as_ref()],
        bump = wager_account.bet_bump,
    )]
    pub wager_account: Account<'info, Wager>,

    #[account(
        mut,
        seeds = [BETTOR_PROFILE, bettor.key().to_bytes().as_ref(), platform_config.key().to_bytes().as_ref()],
        bump = bettor_profile.bettor_bump,
        constraint = bettor_profile.bettor_pubkey == bettor.key() @ MarketError::InvalidAccount
    )]
    pub bettor_profile: Account<'info, Bettor>,

    pub system_program: Program<'info, System>,
}

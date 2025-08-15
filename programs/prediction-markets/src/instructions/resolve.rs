use anchor_lang::prelude::*;

use crate::{
    check_admin, platform_config, Market, MarketError, MarketOutcome, MarketStatus, PlatformConfig, MARKET, PLATFORM_CONFIG
};

#[derive(Accounts)]
pub struct Resolve<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

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
}

impl<'info> Resolve<'info> {
    pub fn resolve_market(&mut self, outcome: MarketOutcome) -> Result<()> {
        check_admin!(self);

        require!(
            Clock::get()?.unix_timestamp >= self.market.dead_line,
            MarketError::MarketNotResolved
        );

        match self.market.market_state {
            crate::MarketStatus::Active => match outcome {
                MarketOutcome::YES => {
                    self.market.market_outcome = MarketOutcome::YES;
                    self.market.market_state = MarketStatus::Resolved;
                }
                MarketOutcome::NO => {
                    self.market.market_outcome = MarketOutcome::NO;
                    self.market.market_state = MarketStatus::Resolved;
                }
                MarketOutcome::NotResolved => {}
            },
            MarketStatus::Resolved => {
                return err!(MarketError::MarketGotResolved)
            }
        }

        Ok(())
    }
}

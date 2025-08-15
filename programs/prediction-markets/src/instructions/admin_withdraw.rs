use anchor_lang::{ prelude::*, system_program::{ Transfer, transfer } };

use crate::{
    check_admin,
    platform_config,
    Market,
    PlatformConfig,
    MARKET,
    MARKET_VAULT,
    PLATFORM_CONFIG,
    TREASURY,
};

use crate::MarketError;

#[derive(Accounts)]
pub struct AdminWithdraw<'info> {
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
        seeds = [MARKET, platform_config.key().to_bytes().as_ref(),&market.market_name.as_bytes()[..32]],
        bump = market.market_bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [MARKET_VAULT,market.key().to_bytes().as_ref()],
        bump = market.market_vault_bump
    )]
    pub market_vault_account: SystemAccount<'info>, // Where bettor desposites there wagers

    #[account(
        mut,
        seeds = [TREASURY,platform_config.key().to_bytes().as_ref()],
        bump = platform_config.treasury_bump
    )]
    pub treasury_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> AdminWithdraw<'info> {
    pub fn admin_withdraw(&mut self) -> Result<()> {
        check_admin!(self);

        let accounts = Transfer {
            from: self.market_vault_account.to_account_info(),
            to: self.treasury_account.to_account_info(),
        };

        let platform_config_seed = self.market.key().to_bytes();
        let seeds = &[
            MARKET_VAULT,
            platform_config_seed.as_ref(),
            &[self.market.market_vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds
        );

        let amount = self.market_vault_account.lamports();

        transfer(ctx, amount)?;

        Ok(())
    }
}

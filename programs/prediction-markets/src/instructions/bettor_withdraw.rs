use anchor_lang::{ prelude::*, system_program::{ Transfer, transfer } };

use crate::{
    check_ban,
    Bettor,
    Market,
    MarketError,
    PlatformConfig,
    Wager,
    BETTOR_PROFILE,
    BETTOR_WALLET,
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
        seeds = [BETTOR_WALLET,bettor.key().to_bytes().as_ref(), platform_config.key().to_bytes().as_ref()],
        bump = bettor_profile.bettor_vault_bump
    )]
    pub bettor_wallet_account: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [BETTOR_PROFILE, bettor.key().to_bytes().as_ref(), platform_config.key().to_bytes().as_ref()],
        bump = bettor_profile.bettor_bump,
        constraint = bettor_profile.bettor_pubkey == bettor.key() @ MarketError::InvalidAccount
    )]
    pub bettor_profile: Account<'info, Bettor>,

    pub system_program: Program<'info, System>,
}

impl<'info> BettorWithdraw<'info> {
    pub fn bettor_withdraw(&mut self) -> Result<()> {
        check_ban!(self.bettor_profile.is_ban);

        let accounts = Transfer {
            from: self.bettor_wallet_account.to_account_info(),
            to: self.bettor.to_account_info(),
        };

        let bettor_seeds = self.bettor.key().to_bytes();
        let platfrom_config_seeds = self.platform_config.key().to_bytes();
        let seeds = &[
            BETTOR_WALLET,
            bettor_seeds.as_ref(),
            platfrom_config_seeds.as_ref(),
            &[self.bettor_profile.bettor_vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds
        );

        transfer(ctx, self.bettor_wallet_account.lamports())?;

        Ok(())
    }
}

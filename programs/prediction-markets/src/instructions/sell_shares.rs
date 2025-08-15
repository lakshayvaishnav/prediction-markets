use anchor_lang::{
    prelude::*,
    solana_program::native_token::LAMPORTS_PER_SOL,
    system_program::{ transfer, Transfer },
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenAccount, TokenInterface, BurnChecked, burn_checked },
};

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
    MARKET_VAULT,
    MINT_NO,
    MINT_YES,
    PLATFORM_CONFIG,
    WAGER,
};
use rust_decimal::prelude::*;

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

impl<'info> SellShares<'info> {
    // burn the givne tokens and transfer the sol to bettor

    pub fn sell_sahres(&mut self, shares_amount: u64, is_yes: bool) -> Result<()> {
        check_ban!(self.bettor_profile.is_ban);

        match self.market.market_state {
            crate::MarketStatus::Active => {
                let sol_amount = match is_yes {
                    true =>
                        self.market
                            .share_calculation(false, shares_amount, 0, self.platform_config.fees)?
                            .to_u64()
                            .ok_or(MarketError::ArithemeticError)?,
                    false =>
                        self.market
                            .share_calculation(false, 0, shares_amount, self.platform_config.fees)?
                            .to_u64()
                            .ok_or(MarketError::ArithemeticError)?,
                };

                self.burn_shares(shares_amount, is_yes)?;
                self.transfer_sol(sol_amount)?;
                self.update_set(shares_amount, is_yes, sol_amount)?;
            }
            crate::MarketStatus::Resolved => {}
        }

        Ok(())
    }

    fn burn_shares(&mut self, outcome_token_amount: u64, is_yes: bool) -> Result<()> {
        let (mint, from) = match is_yes {
            true => (&self.mint_yes, &self.bettor_yes_account),
            false => (&self.mint_no, &self.bettor_no_account),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), BurnChecked {
            authority: self.bettor.to_account_info(),
            mint: mint.to_account_info(),
            from: from.to_account_info(),
        });

        burn_checked(ctx, outcome_token_amount, mint.decimals)?;

        Ok(())
    }

    fn transfer_sol(&mut self, amount_sol: u64) -> Result<()> {
        // transferring sol amount from makret vault account to bettor wallet account

        let accounts = Transfer {
            from: self.market_vault_account.to_account_info(),
            to: self.bettor_wallet_account.to_account_info(),
        };

        let platform_config_seed = self.platform_config.key().to_bytes();
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

        transfer(ctx, amount_sol * LAMPORTS_PER_SOL)?;

        Ok(())
    }

    fn update_set(&mut self, shares_amount: u64, is_yes: bool, amount_earned: u64) -> Result<()> {
        if is_yes {
            self.market.outcome_yes_shares
                .checked_sub(shares_amount)
                .ok_or(MarketError::ArithemeticUnderflow)?;

            self.wager_account.yes_shares
                .checked_sub(shares_amount)
                .ok_or(MarketError::ArithemeticUnderflow)?;
        } else {
            self.market.outcome_no_shares
                .checked_sub(shares_amount)
                .ok_or(MarketError::ArithemeticUnderflow)?;

            self.wager_account.no_shares
                .checked_sub(shares_amount)
                .ok_or(MarketError::ArithemeticUnderflow)?;
        }

        self.wager_account.bet_amount_earned
            .checked_add(amount_earned)
            .ok_or(MarketError::ArithemeticOverflow)?;

        Ok(())
    }
}

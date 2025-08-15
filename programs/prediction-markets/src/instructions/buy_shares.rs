use anchor_lang::{
    prelude::*,
    solana_program::native_token::LAMPORTS_PER_SOL,
    system_program::{ transfer, Transfer },
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::MintToChecked,
    token_interface::{ mint_to_checked, Mint, TokenAccount, TokenInterface },
};

use crate::{
    bettor,
    check_ban,
    check_zero,
    decimal_convo,
    platform_config,
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
pub struct BuyShares<'info> {
    #[account(mut)]
    pub bettor: Signer<'info>,

    #[account(
        mut,
        seeds = [BETTOR_PROFILE, bettor.key().to_bytes().as_ref(), platform_config.key().to_bytes().as_ref()],
        bump = bettor_profile.bettor_bump,
        constraint = bettor_profile.bettor_pubkey == bettor.key() @ MarketError::InvalidAccount
    )]
    pub bettor_profile: Account<'info, Bettor>,

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
        init_if_needed,
        payer = bettor,
        space = Wager::DISCRIMINATOR.len() + Wager::INIT_SPACE,
        seeds = [WAGER, market.key().to_bytes().as_ref(), bettor.key().to_bytes().as_ref()],
        bump
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
        seeds = [MINT_YES,market.key().to_bytes().as_ref()],
        bump = market.mint_yes_bump,
        mint::authority = platform_config,
    )]
    pub mint_yes: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [MINT_NO,market.key().to_bytes().as_ref()],
        bump = market.mint_no_bump,
        mint::authority = platform_config,
    )]
    pub mint_no: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [MARKET_VAULT,market.key().to_bytes().as_ref()],
        bump  = market.market_vault_bump
    )]
    pub market_vault_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = bettor,
        associated_token::authority = bettor,
        associated_token::mint = mint_yes
    )]
    pub bettor_yes_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = bettor,
        associated_token::authority = bettor,
        associated_token::mint = mint_no
    )]
    pub bettor_no_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> BuyShares<'info> {
    pub fn buy_shares(
        &mut self,
        bumps: BuySharesBumps,
        shares_amount: u64,
        is_yes: bool
    ) -> Result<()> {
        check_ban!(self.bettor_profile.is_ban);

        match self.market.market_state {
            crate::MarketStatus::Active => {
                // save data
                if !self.wager_account.is_initialized {
                    self.wager_account.set_inner(Wager {
                        market_pubkey: self.market.key(),
                        bet_amount_spent: 0,
                        bet_amount_earned: 0,
                        market_status: self.market.market_state,
                        market_outcome: crate::MarketOutcome::NotResolved,
                        yes_shares: 0,
                        no_shares: 0,
                        is_initialized: true,
                        bet_bump: bumps.wager_account,
                    });
                }

                check_zero!([decimal_convo!(shares_amount)]);

                // calculate the cost of given shares
                let share_cost = match is_yes {
                    ture =>
                        self.market
                            .share_calculation(true, shares_amount, 0, self.platform_config.fees)?
                            .to_u64()
                            .ok_or(MarketError::ArithemeticError)?,

                    false =>
                        self.market
                            .share_calculation(true, 0, shares_amount, self.platform_config.fees)?
                            .to_u64()
                            .ok_or(MarketError::ArithemeticError)?,
                };

                // left todo
                self.deposit_wager(share_cost)?;
                self.send_shares(shares_amount, is_yes)?;
                self.update_all_state(shares_amount, share_cost, is_yes)?;
            }

            crate::MarketStatus::Resolved => {
                return Err(error!(MarketError::MarketGotResolved));
            }
        }
        Ok(())
    }

    fn deposit_wager(&mut self, amount: u64) -> Result<()> {
        // transfer wager amount from bettor wallet to vault accounts.

        require!(
            self.bettor_wallet_account.lamports() >= amount * LAMPORTS_PER_SOL,
            MarketError::NotEnoughAmount
        );

        let accounts = Transfer {
            from: self.bettor_wallet_account.to_account_info(),
            to: self.market_vault_account.to_account_info(),
        };

        let bettor_seeds = self.bettor.key().to_bytes();
        let platform_config_seeds = self.platform_config.key().to_bytes();

        let seeds = &[
            BETTOR_WALLET,
            bettor_seeds.as_ref(),
            platform_config_seeds.as_ref(),
            &[self.bettor_profile.bettor_vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        transfer(ctx, amount * LAMPORTS_PER_SOL)?;

        Ok(())
    }

    fn send_shares(&mut self, share_amount: u64, is_yes: bool) -> Result<()> {
        // transfering shares to bettor token accounts

        let (to, mint) = match is_yes {
            true => (&self.bettor_yes_account, &self.mint_yes),
            false => (&self.bettor_no_account, &self.mint_no),
        };

        let accounts = MintToChecked {
            to: to.to_account_info(),
            mint: mint.to_account_info(),
            authority: self.platform_config.to_account_info(),
        };

        let seeds = &[PLATFORM_CONFIG, &[self.platform_config.config_bump]];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        mint_to_checked(ctx, share_amount, mint.decimals)?;

        Ok(())
    }

    fn update_all_state(&mut self, share_amount: u64, bet_amount: u64, is_yes: bool) -> Result<()> {
        // update after transferring shares to bettor pubkey

        if is_yes {
            // update the bet account
            self.wager_account.yes_shares = self.wager_account.yes_shares
                .checked_add(share_amount)
                .ok_or(MarketError::ArithemeticOverflow)?;

            // update the market account
            self.market.outcome_yes_shares = self.market.outcome_yes_shares
                .checked_add(share_amount)
                .ok_or(MarketError::ArithemeticOverflow)?;
        } else {
            self.wager_account.no_shares = self.wager_account.no_shares
                .checked_add(share_amount)
                .ok_or(MarketError::ArithemeticOverflow)?;

            self.market.outcome_no_shares = self.market.outcome_no_shares
                .checked_add(share_amount)
                .ok_or(MarketError::ArithemeticOverflow)?;
        }

        self.wager_account.bet_amount_spent = self.wager_account.bet_amount_spent
            .checked_add(bet_amount)
            .ok_or(MarketError::ArithemeticOverflow)?;

        Ok(())
    }
}

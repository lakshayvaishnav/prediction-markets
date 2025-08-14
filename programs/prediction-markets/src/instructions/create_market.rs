use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::system_program::{ transfer, Transfer };
use anchor_spl::metadata::Metadata;
use anchor_spl::token_interface::{ Mint, TokenInterface };

use crate::{ decimal_convo, InitTokenArg, MargetArg, Market, MarketError, PlatformConfig };
use crate::constants::*;
use crate::{ check_admin };
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

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
    pub fn save_market_data(
        &mut self,
        bump: CreateMarketBumps,
        arg: MargetArg,
        metadata_arg: InitTokenArg
    ) -> Result<()> {
        check_admin!(self);

        // CHECK : The liquidity parameter should pass the minimum threshold
        require_gte!(arg.lsmr_b, MINIMUM_LMSR_B, MarketError::ParameterTooLow);

        require!(arg.name.len() < 50, MarketError::MaxLength);

        // initialize the lmsr
        // initialize the market
        self.market.init_market(Market {
            market_name: arg.name,
            description: arg.description,
            initial_deposit: 0,

            lsmr_b: arg.lsmr_b,
            dead_line: arg.dead_line,

            market_state: crate::MarketStatus::Active,
            market_outcome: crate::MarketOutcome::NotResolved,

            outcome_yes_shares: 0,
            outcome_no_shares: 0,

            mint_yes_bump: bump.mint_yes,
            mint_no_bump: bump.mint_no,
            market_vault_bump: bump.market_vault_account,
            market_bump: bump.market,
        });

        // left to implement

        todo!()
    }

    fn deposit_initial_amount(&mut self, lmsr: Market) -> Result<()> {
        let accounts = Transfer {
            from: self.admin.to_account_info(),
            to: self.market_vault_account.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        let decimal_amount = lmsr.cost_calculation(
            &decimal_convo!(lmsr.outcome_yes_shares),
            &decimal_convo!(lmsr.outcome_no_shares)
        )?;

        let amount = decimal_amount.trunc().to_u64().ok_or(MarketError::ArithemeticError)?;

        require!(
            self.admin.to_account_info().lamports() > amount * LAMPORTS_PER_SOL,
            MarketError::NotEnoughAmount
        );

        transfer(ctx, amount * LAMPORTS_PER_SOL)?;

        self.market.initial_deposit = amount;
        
        Ok(())
    }
}

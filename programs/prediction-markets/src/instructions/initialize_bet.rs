use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token };

use crate::{
    Bet,
    Outcome,
    VIRTUAL_NO_SOL_RESERVE,
    VIRTUAL_TOKEN_NO_RESERVE,
    VIRTUAL_TOKEN_YES_RESERVE,
    VIRTUAL_YES_SOL_RESERVE,
};
use crate::error::BetError;

#[derive(Accounts)]
#[instruction(title : String)]
pub struct InitializeBet<'info> {
    #[account(mut)]
    pub bet_creator: Signer<'info>,

    #[account(
        init,
        payer = bet_creator,
        space = Bet::INIT_SPACE,
        seeds = [bet_creator.key().as_ref(), title.as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    // Create YES token mint if it doesn't exist
    #[account(
        init,
        payer = bet_creator,
        seeds = [b"yes_mint", bet.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = bet // Bet PDA mints shares
    )]
    pub yes_token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = bet_creator,
        seeds = [b"no_mint", bet.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = bet
    )]
    pub no_token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeBet<'info> {
    pub fn process(
        &mut self,
        title: String,
        oracle_info: String,
        start_ts: i64,
        end_ts: i64,
        yes_pool: u64,
        no_pool: u64,
        connector_weight: u32
    ) -> Result<()> {
        let bet = &mut self.bet;

        require!(start_ts < end_ts, BetError::InvalidTime);

        bet.set_inner(Bet {
            creator: self.bet_creator.key(),
            bet_title: title,
            oracle_info: oracle_info,
            start_ts: start_ts,
            end_ts: end_ts,
            virtual_yes_sol_reserve: VIRTUAL_YES_SOL_RESERVE,
            virtual_no_sol_reserve: VIRTUAL_NO_SOL_RESERVE,
            virtual_no_token_reserve: VIRTUAL_TOKEN_NO_RESERVE,
            virtual_yes_token_reserve: VIRTUAL_TOKEN_YES_RESERVE,
            total_yes: 0,
            total_no: 0,
            resolved: false,
            outcome: Outcome::Unresolved,
            connector_weight: connector_weight,
        });

        Ok(())
    }
}

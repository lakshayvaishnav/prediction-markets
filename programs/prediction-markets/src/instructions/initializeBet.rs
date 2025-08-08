use anchor_lang::prelude::*;

use crate::{ Bet, Outcome };
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
            yes_pool: yes_pool,
            no_pool: no_pool,
            total_yes: 0,
            total_no: 0,
            resolved: false,
            outcome: Outcome::Unresolved,
            connector_weight: connector_weight,
        });
        
        Ok(())
    }
}

use anchor_lang::prelude::*;

use crate::{ MarketOutcome, MarketStatus };

#[account]
#[derive(InitSpace)]
pub struct Wager {
    pub market_pubkey: Pubkey,

    pub bet_amount_spent: u64,
    pub bet_amount_earned: u64,

    pub market_status: MarketStatus,
    pub market_outcome: MarketOutcome,

    pub yes_shares: u64,
    pub no_shares: u64,

    pub is_initialized: bool,
    pub bet_bump: u8,
}

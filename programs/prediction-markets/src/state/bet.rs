use anchor_lang::prelude::*;

use crate::Outcome;

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub creator: Pubkey,
    #[max_len(100)]
    pub bet_title: String,
    #[max_len(500)]
    pub oracle_info: String,
    pub start_ts: i64,
    pub end_ts: i64,
    pub virtual_yes_sol_reserve: u64,
    pub virtual_no_sol_reserve: u64,
    pub virtual_yes_token_reserve: u64,
    pub virtual_no_token_reserve: u64,
    pub total_yes: u64,
    pub total_no: u64,
    pub resolved: bool,
    pub outcome: Outcome,
    pub connector_weight: u32, // stored in parts per million (e.g. ppm where 1_000_000 == 100%)
}

//Pick a weight based on how aggressively you want the marginal price to move. (connector_weight)

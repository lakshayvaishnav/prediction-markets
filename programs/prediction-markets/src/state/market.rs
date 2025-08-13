use anchor_lang::prelude::*;

use crate::{MarketOutcome, MarketStatus};

#[account]
#[derive(InitSpace)]
pub struct Market {
    #[max_len(200)]
    pub market_name: String,

    #[max_len(500)]
    pub description : String,

    pub initial_deposit : u64,
    pub lsmr_b : u64,
    pub dead_line : i64,

    pub market_state : MarketStatus,
    pub market_outcome : MarketOutcome,

    pub outcome_yes_shares : u64,
    pub outcome_no_shares : u64,

    pub mint_yes_bump : u8,
    pub mint_no_bump : u8,
    pub market_vault_bump : u8,
    pub market_bump : u8
}
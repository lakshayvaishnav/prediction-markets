use anchor_lang::prelude::*;

#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Clone, Copy)]
pub enum MarketStatus {
    Resolved,
    Active,
}

#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum MarketOutcome {
    YES,
    NO,
    NotResolved,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct MargetArg {
    pub name: String,
    pub description: String,
    pub lsmr_b: u64,
    pub dead_line: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenArg {
    pub yes_name: String,
    pub yes_symbol: String,
    pub yes_uri: String,

    pub no_name: String,
    pub no_symbol: String,
    pub no_uri: String,
}

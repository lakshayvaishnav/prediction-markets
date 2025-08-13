use anchor_lang::prelude::*;


#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Clone, Copy)]
pub enum MarketStatus {
    Resolved,
    Active
}

#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum MarketOutcome {
    YES,
    NO,
    NotResolved
}


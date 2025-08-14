use anchor_lang::prelude::*;
use rust_decimal::{ Decimal, MathematicalOps };
use crate::MarketError;

use crate::{ add_or_sub, decimal_convo, div, mul, Market };

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

impl Market {
    pub fn init_market(&mut self, arg: Market) -> &Self {
        self.market_name = arg.market_name;
        self.description = arg.description;

        // LMSR liquidity
        self.lsmr_b = arg.lsmr_b;
        self.dead_line = arg.dead_line;

        self.market_state = arg.market_state;
        self.market_outcome = arg.market_outcome;

        // shares
        self.outcome_no_shares = arg.outcome_no_shares;
        self.outcome_yes_shares = arg.outcome_yes_shares;

        // bumps
        self.market_bump = arg.market_bump;
        self.mint_no_bump = arg.mint_no_bump;
        self.mint_yes_bump = arg.mint_yes_bump;
        self.market_vault_bump = arg.market_vault_bump;

        return self;
    }

    // cost calculations
    pub fn cost_calculation(&self, yes_shares: &Decimal, no_shares: &Decimal) -> Result<Decimal> {
        // cost function = b.ln(e.pow(q1/b)) + e.pow(q2/b)

        let outcome_yes = div!(yes_shares, decimal_convo!(self.lsmr_b)).exp(); // e.powf(q1/b)
        let outcome_no = div!(no_shares, decimal_convo!(self.lsmr_b)).exp(); // e.powf(q2/b)
        let outcome_sum = add_or_sub!(outcome_yes, outcome_no, true)?;

        let cost = mul!(outcome_sum.ln(), decimal_convo!(self.lsmr_b));

        Ok(cost)
    }
}

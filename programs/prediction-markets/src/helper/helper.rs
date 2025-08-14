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
        let outcome_sum = add_or_sub!(outcome_yes, outcome_no, true)?; // e.pow(q1/b) + e.powf(q2/b)

        let cost = mul!(outcome_sum.ln(), decimal_convo!(self.lsmr_b)); // b.ln(e.pow(q1/b) + e.pow(q2/b))

        Ok(cost)
    }

    pub fn share_calculation(
        &self,
        is_buy: bool,
        yes_shares: u64,
        no_shares: u64,
        fee_bps: u16
    ) -> Result<Decimal> {
        // Delta C = C2 - C1; (new cost function - current cost function)
        let current_cost = self.cost_calculation(
            &decimal_convo!(self.outcome_yes_shares),
            &decimal_convo!(self.outcome_no_shares)
        )?;

        // q2 + q1(for buying) & q2 - q1 (for selling)
        let new_yes = add_or_sub!(
            decimal_convo!(self.outcome_yes_shares),
            decimal_convo!(yes_shares),
            is_buy
        )?;

        let new_no = add_or_sub!(
            decimal_convo!(self.outcome_no_shares),
            decimal_convo!(no_shares),
            is_buy
        )?;

        let new_cost = self.cost_calculation(&new_yes, &new_no)?;

        let delta_cost = match is_buy {
            true => { add_or_sub!(new_cost, current_cost, false)? }
            false => { add_or_sub!(current_cost, new_cost, true)? }
        };

        let share_cost = self.fees_calculation(fee_bps, delta_cost, is_buy)?;

        Ok(share_cost)
    }

    fn fees_calculation(&self, fee_bps: u16, delta_cost: Decimal, is_buy: bool) -> Result<Decimal> {
        require!(fee_bps < 10000 && fee_bps != 0, MarketError::InvalidFees);

        let fee_multiplier = div!(decimal_convo!(fee_bps), decimal_convo!(10000));
        let fee_amount = mul!(delta_cost, fee_multiplier);

        let cost = add_or_sub!(delta_cost, fee_amount, is_buy)?;

        Ok(cost)
    }
}

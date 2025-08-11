use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;
pub mod calc;
pub mod constants;

pub use constants::*;
pub use calc::*;
pub use error::*;
pub use state::*;
pub use instructions::*;

declare_id!("5msyCUKpqLzmUjrWtBAJRGxTdfNsw9YZgPnV6Ei9yp58");

#[program]
pub mod prediction_markets {
    use super::*;

    pub fn InitializeBet(
        ctx: Context<InitializeBet>,
        title: String,
        oracle_info: String,
        start_ts: i64,
        end_ts: i64,
        yes_pool: u64,
        no_pool: u64,
        connector_weight: u32
    ) -> Result<()> {
        ctx.accounts.process(
            title,
            oracle_info,
            start_ts,
            end_ts,
            yes_pool,
            no_pool,
            connector_weight
        )
    }
}

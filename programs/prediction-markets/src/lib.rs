use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;
pub mod helper;

pub use helper::*;
pub use error::*;
pub use state::*;
pub use instructions::*;

declare_id!("5msyCUKpqLzmUjrWtBAJRGxTdfNsw9YZgPnV6Ei9yp58");

#[program]
pub mod prediction_markets {
    use super::*;
}

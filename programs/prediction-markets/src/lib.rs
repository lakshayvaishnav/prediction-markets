use anchor_lang::prelude::*;

pub mod state;
pub mod error;

pub use error::*;
pub use state::*;

declare_id!("2wanrg4dKZJW3Hzo6e8jBuxh2gDSVwz8JjnZA5SF7jXT");

#[program]
pub mod prediction_markets {
    use super::*;
}

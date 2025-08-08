use anchor_lang::prelude::*;

#[error_code]
pub enum BetError {
    #[msg("Bet start time must be before end time")]
    InvalidTime,
    #[msg("Betting window closed or not started")]
    OutsideBettingWindow,
    #[msg("Bet not yet resolved")]
    NotResolved,
    #[msg("Outcome mismatch for this side")]
    WrongOutcome,
    #[msg("Cannot resolve before end time unless creator")]
    CannotResolveYet,
    #[msg("Bonding curve calculation error")]
    CurveError,
}

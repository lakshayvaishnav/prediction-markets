use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum Outcome {
    Unresolved,
    Yes,
    No,
}

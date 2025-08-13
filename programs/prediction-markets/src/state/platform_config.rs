use anchor_lang::prelude::*;

#[account]
pub struct PlatformConfig {
    pub admin: Vec<Pubkey>,
    pub fees: u16,

    pub treasury_amount: u64,
    pub treasury_bump: u8,
    pub config_bump: u8,

    pub is_initialized: bool,
}

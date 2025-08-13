use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlatformConfig {
    #[max_len(3)]
    pub admin: Vec<Pubkey>,
    pub fees: u16,

    pub treasury_amount: u64,
    pub treasury_bump: u8,
    pub config_bump: u8,

    pub is_initialized: bool,
}

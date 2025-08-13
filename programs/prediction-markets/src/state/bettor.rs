use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bettor {
    pub bettor_pubkey: Pubkey,

    #[max_len(50)]
    pub bettor_name: String,

    pub bettor_net_profit: i64, // might be in loss or profit
    pub balance: u64,

    pub is_ban: bool,

    pub bettor_vault_bump: u8,
    pub bettor_bump: u8,
}

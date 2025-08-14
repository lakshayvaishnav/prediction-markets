use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token_interface::TokenInterface };

use crate::{ bettor, platform_config, Bettor, MarketError, Wager, BETTOR_PROFILE };

#[derive(Accounts)]
pub struct BuyShares<'info> {
    #[account(mut)]
    pub bettor: Signer<'info>,

    #[account(
        mut,
        seeds = [BETTOR_PROFILE, bettor.key().to_bytes().as_ref(), platform_config.key().to_bytes().as_ref()],
        bump = bettor_profile.bettor_bump,
        constraint = bettor_profile.bettor_pubkey == bettor.key() @ MarketError::InvalidAccount
    )]
    pub bettor_profile: Account<'info, Bettor>,

    #[account(
        init_if_needed,
        payer = bettor,
        space = Wager::DISCRIMINATOR.len() + Wager::INIT_SPACE
    )]
    pub wager_account: Account<'info, Wager>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

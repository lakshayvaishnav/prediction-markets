use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount } };

use crate::Bet;

#[derive(Accounts)]
#[instruction(title:String)]
pub struct BuyShares<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK : it is safe boii
    pub bet_creator: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [bet_creator.key().as_ref(), title.as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        mut,
        seeds = [b"yes_mint", bet.key().as_ref()],
        bump, 
    )]
    pub yes_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"no_mint", bet.key().as_ref()],
        bump
    )]
    pub no_token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = yes_token_mint,
        associated_token::authority = user
    )]
    pub user_yes_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = no_token_mint,
        associated_token::authority = user
    )]
    pub user_no_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyShares<'info> {
    pub fn process(ctx: Context<BuyShares>) -> Result<()> {
        Ok(())
    }
}

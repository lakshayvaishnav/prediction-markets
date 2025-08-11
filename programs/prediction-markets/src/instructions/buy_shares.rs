use anchor_lang::{ prelude::*, solana_program::{ system_instruction, program::invoke } };
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, MintTo, Token, TokenAccount },
};

use crate::{
    calculate_purchase_return,
    Bet,
    Side,
    CONNECTOR_WEIGHT,
    VIRTUAL_SOL_RESERVE,
    VIRTUAL_TOKEN_NO_RESERVE,
    VIRTUAL_TOKEN_YES_RESERVE,
};

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
    pub fn process(
        ctx: Context<BuyShares>,
        direction: u8,
        deposit_amount: u64,
        side: Side,
        title: String,
        bump: &BuySharesBumps
    ) -> Result<()> {
        // TODO : deduct the platform fees.

        // transfer the sol to the bet contract.
        let ix = system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.bet.key(),
            deposit_amount
        );

        invoke(&ix, &[ctx.accounts.user.to_account_info(), ctx.accounts.bet.to_account_info()])?;

        // mint the shares to the user.
        let cpi_program = ctx.accounts.token_program.to_account_info();

        let seeds: &[&[&[u8]]] = &[
            &[ctx.accounts.bet_creator.key.as_ref(), title.as_bytes(), &[bump.bet]],
        ];

        match side {
            Side::Yes => {
                let amount_yes_token_out = calculate_purchase_return(
                    CONNECTOR_WEIGHT,
                    VIRTUAL_SOL_RESERVE,
                    VIRTUAL_TOKEN_YES_RESERVE,
                    deposit_amount
                );

                let cpi_context = CpiContext::new_with_signer(
                    cpi_program,
                    MintTo {
                        authority: ctx.accounts.bet.to_account_info(),
                        mint: ctx.accounts.yes_token_mint.to_account_info(),
                        to: ctx.accounts.user_yes_ata.to_account_info(),
                    },
                    seeds
                );

                token::mint_to(cpi_context, amount_yes_token_out)?;

                // update reserves
                ctx.accounts.bet.total_yes = ctx.accounts.bet.total_yes
                    .checked_add(amount_yes_token_out)
                    .unwrap();
            }

            Side::No => {
                let amount_no_token_out = calculate_purchase_return(
                    CONNECTOR_WEIGHT,
                    VIRTUAL_SOL_RESERVE,
                    VIRTUAL_TOKEN_NO_RESERVE,
                    deposit_amount
                );

                let accounts = MintTo {
                    authority: ctx.accounts.bet.to_account_info(),
                    mint: ctx.accounts.no_token_mint.to_account_info(),
                    to: ctx.accounts.user_no_ata.to_account_info(),
                };
                let cpi_context = CpiContext::new_with_signer(cpi_program, accounts, seeds);
                token::mint_to(cpi_context, amount_no_token_out)?;
                // update reserves
                ctx.accounts.bet.total_no = ctx.accounts.bet.total_no
                    .checked_add(amount_no_token_out)
                    .unwrap();
            }
        }

        Ok(())
    }
}

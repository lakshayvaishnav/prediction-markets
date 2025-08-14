use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };
use crate::{ Bettor, PlatformConfig, BETTOR_PROFILE, PLATFORM_CONFIG, BETTOR_WALLET };

#[derive(Accounts)]
pub struct InitializeBettor<'info> {
    #[account(mut)]
    pub bettor: Signer<'info>,

    #[account(
        mut,
        seeds = [PLATFORM_CONFIG],
        bump = platform_config.config_bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    #[account(
        init,
        payer = bettor,
        space = Bettor::DISCRIMINATOR.len() + Bettor::INIT_SPACE,
        seeds = [
            BETTOR_PROFILE,
            bettor.key().to_bytes().as_ref(),
            platform_config.key().to_bytes().as_ref(),
        ],
        bump
    )]
    pub bettor_profile: Account<'info, Bettor>,

    #[account(
        mut,
        seeds =  [BETTOR_WALLET ,bettor.key().to_bytes().as_ref(), platform_config.key().to_bytes().as_ref() ],
        bump
    )]
    pub bettor_wallet_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeBettor<'info> {
    pub fn init_bettor(
        &mut self,
        name: Option<String>,
        amount_deposit: u64,
        bumps: InitializeBettorBumps
    ) -> Result<()> {
        self.bettor_profile.set_inner(Bettor {
            bettor_pubkey: self.bettor.key(),
            bettor_name: name,
            bettor_net_profit: 0,
            balance: 0,
            is_ban: false,
            bettor_vault_bump: bumps.bettor_wallet_account,
            bettor_bump: bumps.bettor_profile,
        });

        self.transfer_bettor_funds(amount_deposit)?;

        Ok(())
    }

    fn transfer_bettor_funds(&mut self, amount_deposit: u64) -> Result<()> {
        // transfer bettor funds to platfrom's wallet

        // TODO : ADD MACRO to check amount != 0

        let accounts = Transfer {
            from: self.bettor.to_account_info(),
            to: self.bettor_wallet_account.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, amount_deposit)?;

        // update the bettor balance
        self.bettor_profile.balance = self.bettor_profile.balance.checked_add(amount_deposit).unwrap();
        Ok(())    
    }

}


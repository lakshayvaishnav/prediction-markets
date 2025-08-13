use anchor_lang::prelude::*;

use crate::{ MarketError, PlatformConfig, TREASURY };
use crate::constants::PLATFORM_CONFIG;
#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = PlatformConfig::DISCRIMINATOR.len() + PlatformConfig::INIT_SPACE,
        seeds = [PLATFORM_CONFIG],
        bump
    )]
    pub platfrom_config: Account<'info, PlatformConfig>,

    #[account(seeds = [TREASURY, platfrom_config.key().to_bytes().as_ref()], bump)]
    pub treasury_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitConfig<'info> {
    pub fn init_config(&mut self, bumps: InitConfigBumps, fees: Option<u16>) -> Result<()> {
        if !self.platfrom_config.is_initialized {
            self.platfrom_config.is_initialized = true;
            self.platfrom_config.fees = fees.unwrap();
            self.platfrom_config.treasury_bump = bumps.treasury_account;
            self.platfrom_config.config_bump = bumps.platfrom_config;
            self.platfrom_config.treasury_amount = 0;
        }

        // chcks for more then two admins
        require!(self.platfrom_config.admin.len() < 2, MarketError::TooManyAdmins);

        let admin_check = self.platfrom_config.admin
            .iter()
            .any(|admin_pubkey| admin_pubkey == self.admin.key);

        //
        require!(!admin_check, MarketError::AdminExist);

        self.platfrom_config.admin.push(self.admin.key());

        Ok(())
    }
}

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{ADMIN_PUBKEYS, VAULT_SEED_ADMIN},
    errors::ErrorCodes,
};

#[derive(Accounts)]
pub struct AdminWithdraw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED_ADMIN],
        bump
    )]
    pub admin_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'a> AdminWithdraw<'a> {
    pub fn admin_withdraw(&mut self, amount: u64, bumps: &AdminWithdrawBumps) -> Result<()> {
        require!(
            self.admin_vault.lamports() >= amount,
            ErrorCodes::InsufficientFundsInVault
        );
        require!(
            ADMIN_PUBKEYS.contains(&self.admin.key()),
            ErrorCodes::UnauthorizedAdmin
        );

        let cpi_program = self.system_program.to_account_info();
        let admin_vault = self.admin_vault.to_account_info();
        let admin = self.admin.to_account_info();
        let cpi_account_options = Transfer {
            from: admin_vault,
            to: admin,
        };
        let signer_seeds: &[&[&[u8]]] = &[&[VAULT_SEED_ADMIN, &[bumps.admin_vault]]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account_options, signer_seeds);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

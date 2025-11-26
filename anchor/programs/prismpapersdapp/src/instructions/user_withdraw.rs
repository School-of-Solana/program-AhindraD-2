use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{constants::VAULT_SEED_USER, errors::ErrorCodes};

#[derive(Accounts)]
pub struct UserWithdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED_USER, user.key().as_ref()],
        bump
    )]
    pub user_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'a> UserWithdraw<'a> {
    pub fn user_withdraw(&mut self, amount: u64, bumps: &UserWithdrawBumps) -> Result<()> {
        require!(
            self.user_vault.lamports() >= amount,
            ErrorCodes::InsufficientFundsInVault
        );

        let cpi_program = self.system_program.to_account_info();
        let user_vault = self.user_vault.to_account_info();
        let user = self.user.to_account_info();
        let cpi_account_options = Transfer {
            from: user_vault,
            to: user,
        };
        let binding = self.user.key();
        let signer_seeds: &[&[&[u8]]] =
            &[&[VAULT_SEED_USER, binding.as_ref(), &[bumps.user_vault]]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account_options, signer_seeds);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

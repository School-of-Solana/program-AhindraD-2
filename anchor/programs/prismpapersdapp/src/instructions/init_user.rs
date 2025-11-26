use anchor_lang::prelude::*;

use crate::{
    constants::{
        ANCHOR_DISCRIMINATOR, USER_INIT_STAT, USER_NAME_MAX_LENGTH, USER_SEED, VAULT_SEED_USER,
    },
    errors::ErrorCodes,
    states::User,
};

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR + User::INIT_SPACE,
        seeds = [USER_SEED, owner.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,

    #[account(
        seeds = [VAULT_SEED_USER, owner.key().as_ref()],
        bump
    )]
    pub user_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'a> InitUser<'a> {
    pub fn initiate_user(&mut self, name: String, bumps: &InitUserBumps) -> Result<()> {
        require!(
            name.len() > 0 && name.len() < USER_NAME_MAX_LENGTH,
            ErrorCodes::UserNameInvalid
        );
        let owner = self.owner.key();

        self.user_account.set_inner(User {
            owner,
            name,
            published: USER_INIT_STAT,
            purchased: USER_INIT_STAT,
            reviewed: USER_INIT_STAT,
            sold: USER_INIT_STAT,
            earning: 0u64,
            timestamp: Clock::get()?.unix_timestamp,
            bump: bumps.user_account,
        });

        Ok(())
    }
}

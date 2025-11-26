use anchor_lang::prelude::*;

use crate::{
    constants::{
        ANCHOR_DISCRIMINATOR, ENCRYPTION_KEY_MAX_LENGTH, PAPER_DESCRIPTION_MAX_LENGTH,
        PAPER_INIT_STAT, PAPER_SEED, PAPER_TITLE_MAX_LENGTH, PAPER_URL_MAX_LENGTH, USER_SEED,
    },
    errors::ErrorCodes,
    states::{ResearchPaper, User},
};

#[derive(Accounts)]
#[instruction(title: String)]
pub struct InitResearch<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        init,
        payer = author,
        space = ANCHOR_DISCRIMINATOR + ResearchPaper::INIT_SPACE,
        seeds = [PAPER_SEED, author.key().as_ref()],
        bump
    )]
    pub research_paper: Account<'info, ResearchPaper>,

    #[account(
        mut,
        seeds = [USER_SEED, user_account.owner.key().as_ref()],
        bump=user_account.bump
    )]
    pub user_account: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'a> InitResearch<'a> {
    pub fn initiate_research(
        &mut self,
        title: String,
        description: String,
        price: u64,
        encrypted_url: String,
        encryption_key: String,
        bumps: &InitResearchBumps,
    ) -> Result<()> {
        require!(
            title.len() > 0 && title.len() < PAPER_TITLE_MAX_LENGTH,
            ErrorCodes::PaperTitleInvalid
        );
        require!(
            description.len() > 0 && description.len() < PAPER_DESCRIPTION_MAX_LENGTH,
            ErrorCodes::PaperDescriptionInvalid
        );
        require!(
            encrypted_url.len() > 0 && encrypted_url.len() < PAPER_URL_MAX_LENGTH,
            ErrorCodes::PaperUrlEmptyOrTooLong
        );
        require!(
            encryption_key.len() > 0 && encryption_key.len() < ENCRYPTION_KEY_MAX_LENGTH,
            ErrorCodes::EncryptionKeyEmptyOrTooLong
        );
        require!(price > 0, ErrorCodes::ResearchPriceInvalid);

        let author = self.author.key();

        //updating the states
        self.research_paper.set_inner(ResearchPaper {
            author,
            title,
            description,
            price,
            sales: PAPER_INIT_STAT,
            reviews: PAPER_INIT_STAT,
            encrypted_url,
            encryption_key,
            timestamp: Clock::get()?.unix_timestamp,
            bump: bumps.research_paper,
        });
        self.user_account
            .published
            .checked_add(1u16)
            .ok_or(ErrorCodes::MathOverflow)?;
        Ok(())
    }
}

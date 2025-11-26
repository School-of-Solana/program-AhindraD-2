use anchor_lang::prelude::*;

use crate::{
    constants::{
        ENCRYPTION_KEY_MAX_LENGTH, PAPER_DESCRIPTION_MAX_LENGTH, PAPER_SEED,
        PAPER_TITLE_MAX_LENGTH, PAPER_URL_MAX_LENGTH,
    },
    errors::ErrorCodes,
    states::ResearchPaper,
};

#[derive(Accounts)]
pub struct UpdateResearch<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        mut,
        seeds = [PAPER_SEED, research_paper.author.key().as_ref()],
        bump=research_paper.bump
    )]
    pub research_paper: Account<'info, ResearchPaper>,
}

impl<'a> UpdateResearch<'a> {
    pub fn update_research(
        &mut self,
        title: String,
        description: String,
        price: u64,
        encrypted_url: String,
        encryption_key: String,
    ) -> Result<()> {
        require!(
            self.author.key() == self.research_paper.author,
            ErrorCodes::UnauthorizedUpdate
        );
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
        require!(price > 0, ErrorCodes::ResearchPriceInvalid);
        require!(
            encryption_key.len() > 0 && encryption_key.len() < ENCRYPTION_KEY_MAX_LENGTH,
            ErrorCodes::EncryptionKeyEmptyOrTooLong
        );

        self.research_paper.title = title;
        self.research_paper.description = description;
        self.research_paper.price = price;
        self.research_paper.encrypted_url = encrypted_url;
        self.research_paper.encryption_key = encryption_key;
        Ok(())
    }
}

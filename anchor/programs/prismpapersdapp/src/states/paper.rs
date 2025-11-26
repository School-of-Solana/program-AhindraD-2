use anchor_lang::prelude::*;

use crate::constants::{
    ENCRYPTION_KEY_MAX_LENGTH, PAPER_DESCRIPTION_MAX_LENGTH, PAPER_TITLE_MAX_LENGTH,
    PAPER_URL_MAX_LENGTH,
};

#[account]
#[derive(InitSpace)]
pub struct ResearchPaper {
    pub author: Pubkey,
    #[max_len(PAPER_TITLE_MAX_LENGTH)]
    pub title: String,
    #[max_len(PAPER_DESCRIPTION_MAX_LENGTH)]
    pub description: String,
    pub price: u64,
    pub sales: u32,
    pub reviews: u32,
    #[max_len(PAPER_URL_MAX_LENGTH)]
    pub encrypted_url: String,
    #[max_len(ENCRYPTION_KEY_MAX_LENGTH)]
    pub encryption_key: String,
    pub timestamp: i64,
    pub bump: u8,
}

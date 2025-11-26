use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ResearchPaper {
    pub author: Pubkey,
    #[max_len(100)]
    pub title: String,
    #[max_len(400)]
    pub description: String,
    pub price: u64,
    pub sales: u32,
    pub reviews: u32,
    #[max_len(200)]
    pub encrypted_url: String,
    #[max_len(300)]
    pub encryption_key: String,
    pub timestamp: i64,
    pub bump: u8,
}

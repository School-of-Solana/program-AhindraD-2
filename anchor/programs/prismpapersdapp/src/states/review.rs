use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PeerReview {
    pub reviewer: Pubkey,
    pub reviewed_paper: Pubkey,
    #[max_len(200)]
    pub review_url: String,
    pub status: ReviewStatus,
    pub proposed_reward: u64,
    pub timestamp: i64,
    pub bump: u8,
}

//A enum to represent the status of a submitted peer review
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum ReviewStatus {
    Pending,
    Accepted,
    Rejected,
}

use anchor_lang::prelude::*;

use crate::{
    constants::{ANCHOR_DISCRIMINATOR, PAPER_SEED, RECEIPT_SEED, REVIEW_SEED, REVIEW_URL_MAX_LENGTH, USER_SEED},
    errors::ErrorCodes,
    states::{AccessReceipt, PeerReview, ResearchPaper, ReviewStatus, User},
};

#[derive(Accounts)]
pub struct ReviewPaper<'info> {
    #[account(mut)]
    pub reviewer: Signer<'info>,

    #[account(
        mut,
        seeds = [PAPER_SEED, research_paper.author.key().as_ref()],
        bump=research_paper.bump
    )]
    pub research_paper: Account<'info, ResearchPaper>,

    //checking if the reviewer has already purchased the paper, as only purchasers can review
    #[account(
        seeds = [
            RECEIPT_SEED, 
            reviewer.key().as_ref(), 
            research_paper.key().as_ref()
        ],
        bump = access_receipt.bump
    )]
    pub access_receipt: Account<'info, AccessReceipt>,

    #[account(
        mut,
        seeds = [USER_SEED, reviewer_user_account.owner.key().as_ref()],
        bump=reviewer_user_account.bump
    )]
    pub reviewer_user_account: Account<'info, User>,

    #[account(
        init,
        payer = reviewer,
        space = ANCHOR_DISCRIMINATOR + PeerReview::INIT_SPACE,
        seeds = [REVIEW_SEED, reviewer.key().as_ref(), research_paper.key().as_ref()],
        bump
    )]
    pub peer_review: Account<'info, PeerReview>,

    pub system_program: Program<'info, System>,
}

impl<'a> ReviewPaper<'a> {
    pub fn review_paper(&mut self, review_url: String, proposed_reward: u64, bumps: &ReviewPaperBumps) -> Result<()> {
        require!(
            self.access_receipt.buyer == self.reviewer.key(),
            ErrorCodes::PaperNotPurchased
        );
        require!(
            review_url.len() > 0 && review_url.len() < REVIEW_URL_MAX_LENGTH,
            ErrorCodes::ReviewUrlEmpty
        );
        require!(
            self.research_paper.author != self.reviewer.key(),
            ErrorCodes::AuthorCantReviewSelf
        );

        //storing the review
        let reviewer = self.reviewer.key();
        let reviewed_paper = self.research_paper.key();
        self.peer_review.set_inner(PeerReview {
            reviewer,
            reviewed_paper,
            review_url,
            status: ReviewStatus::Pending,
            proposed_reward,
            timestamp: Clock::get()?.unix_timestamp,
            bump: bumps.peer_review,
        });
        //updating the states
        self.research_paper
            .reviews
            .checked_add(1u32)
            .ok_or(ErrorCodes::MathOverflow)?;
        self.reviewer_user_account
            .reviewed
            .checked_add(1u16)
            .ok_or(ErrorCodes::MathOverflow)?;

        Ok(())
    }
}

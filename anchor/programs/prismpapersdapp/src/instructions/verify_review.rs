use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{
        GLOBAL_FEE_PERCENTAGE, PAPER_SEED, REVIEW_SEED, USER_SEED, VAULT_SEED_ADMIN,
        VAULT_SEED_USER,
    },
    errors::ErrorCodes,
    states::{PeerReview, ResearchPaper, ReviewStatus, User},
};

#[derive(Accounts)]
pub struct VerifyReview<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        mut,
        seeds = [REVIEW_SEED, peer_review.reviewer.key().as_ref(), peer_review.reviewed_paper.key().as_ref()],
        bump=peer_review.bump
    )]
    pub peer_review: Account<'info, PeerReview>,

    #[account(
        mut,
        seeds = [PAPER_SEED, peer_review.reviewed_paper.key().as_ref()],
        bump=research_paper.bump
    )]
    pub research_paper: Account<'info, ResearchPaper>,

    #[account(
        mut,
        seeds = [USER_SEED, reviewer_user_account.owner.key().as_ref()],
        bump=reviewer_user_account.bump
    )]
    pub reviewer_user_account: Account<'info, User>,

    #[account(
        mut,
        seeds = [USER_SEED, author_user_account.owner.key().as_ref()],
        bump=author_user_account.bump
    )]
    pub author_user_account: Account<'info, User>,

    #[account(
        mut,
        seeds = [VAULT_SEED_USER, reviewer_user_account.owner.key().as_ref()],
        bump
    )]
    pub reviewer_vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED_USER, author.key().as_ref()],
        bump
    )]
    pub author_vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED_ADMIN],
        bump
    )]
    pub admin_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'a> VerifyReview<'a> {
    pub fn verify_review(&mut self, accept_proposed_review: bool) -> Result<()> {
        require!(
            self.peer_review.status == ReviewStatus::Pending,
            ErrorCodes::ReviewNotPending
        );
        require!(
            self.author.key() == self.research_paper.author,
            ErrorCodes::UnauthorizedUpdate
        );

        if accept_proposed_review {
            let total_amount = self.peer_review.proposed_reward;
            require!(
                self.author.lamports() >= total_amount,
                ErrorCodes::InsufficientFundsInWallet
            );
            let platform_fee = total_amount
                .checked_mul(GLOBAL_FEE_PERCENTAGE)
                .ok_or(ErrorCodes::MathOverflow)?
                .checked_div(100)
                .ok_or(ErrorCodes::MathOverflow)?;
            let reviewer_earning = total_amount
                .checked_sub(platform_fee)
                .ok_or(ErrorCodes::MathOverflow)?;
            //transferring the reward to the reviewer vault
            let cpi_program = self.system_program.to_account_info();
            let author = self.author.to_account_info();
            let reviewer_vault = self.reviewer_vault.to_account_info();
            let cpi_account_options = Transfer {
                from: author,
                to: reviewer_vault,
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_account_options);
            transfer(cpi_ctx, reviewer_earning)?;

            //transferring the platform fee to the admin vault
            let cpi_program = self.system_program.to_account_info();
            let author = self.author.to_account_info();
            let admin_vault = self.admin_vault.to_account_info();
            let cpi_account_options = Transfer {
                from: author,
                to: admin_vault,
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_account_options);
            transfer(cpi_ctx, platform_fee)?;

            self.peer_review.status = ReviewStatus::Accepted;
            self.reviewer_user_account.earning = self
                .reviewer_user_account
                .earning
                .checked_add(reviewer_earning)
                .ok_or(ErrorCodes::MathOverflow)?;
        } else {
            self.peer_review.status = ReviewStatus::Rejected;
        }
        Ok(())
    }
}

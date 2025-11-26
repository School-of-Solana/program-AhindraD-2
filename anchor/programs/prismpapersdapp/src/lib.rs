#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
use instructions::*;
mod states;
declare_id!("2nvhRn83KBxkkAfLH64meTq8cYB5aRLnZVbsxZdgfPTv");

#[program]
pub mod prismpapersdapp {
    use super::*;

    pub fn init_user(ctx: Context<InitUser>, name: String) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.initiate_user(name, &bumps)
    }

    pub fn init_research(
        ctx: Context<InitResearch>,
        title: String,
        description: String,
        price: u64,
        encrypted_url: String,
        encryption_key: String,
    ) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.initiate_research(
            title,
            description,
            price,
            encrypted_url,
            encryption_key,
            &bumps,
        )
    }
    pub fn update_research(
        ctx: Context<UpdateResearch>,
        title: String,
        description: String,
        price: u64,
        encrypted_url: String,
        encryption_key: String,
    ) -> Result<()> {
        ctx.accounts
            .update_research(title, description, price, encrypted_url, encryption_key)
    }

    pub fn purchase_access(ctx: Context<PurchaseAccess>) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.purchase_access(&bumps)
    }
    pub fn review_paper(
        ctx: Context<ReviewPaper>,
        review_url: String,
        proposed_reward: u64,
    ) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts
            .review_paper(review_url, proposed_reward, &bumps)
    }

    pub fn verify_review(ctx: Context<VerifyReview>, accept_proposed_review: bool) -> Result<()> {
        ctx.accounts.verify_review(accept_proposed_review)
    }

    pub fn user_withdraw(ctx: Context<UserWithdraw>, amount: u64) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.user_withdraw(amount, &bumps)
    }

    pub fn admin_withdraw(ctx: Context<AdminWithdraw>, amount: u64) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.admin_withdraw(amount, &bumps)
    }
}

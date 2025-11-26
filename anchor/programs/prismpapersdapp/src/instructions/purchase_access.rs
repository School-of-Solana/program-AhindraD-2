use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{
        ANCHOR_DISCRIMINATOR, GLOBAL_FEE_PERCENTAGE, PAPER_SEED, RECEIPT_SEED, USER_SEED,
        VAULT_SEED_ADMIN, VAULT_SEED_USER,
    },
    errors::ErrorCodes,
    states::{AccessReceipt, ResearchPaper, User},
};

#[derive(Accounts)]
pub struct PurchaseAccess<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [PAPER_SEED, research_paper.author.key().as_ref()],
        bump=research_paper.bump
    )]
    pub research_paper: Account<'info, ResearchPaper>,

    #[account(
        mut,
        seeds = [USER_SEED, buyer_user_account.owner.key().as_ref()],
        bump=buyer_user_account.bump
    )]
    pub buyer_user_account: Account<'info, User>,

    #[account(
        mut,
        seeds = [VAULT_SEED_USER, buyer.key().as_ref()],
        bump
    )]
    pub buyer_vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [USER_SEED, research_paper.author.key().as_ref()],
        bump=author_user_account.bump
    )]
    pub author_user_account: Account<'info, User>,
    #[account(
        mut,
        seeds = [VAULT_SEED_USER, research_paper.author.key().as_ref()],
        bump
    )]
    pub author_vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED_ADMIN],
        bump
    )]
    pub admin_vault: SystemAccount<'info>,

    #[account(
        init,
        payer = buyer,
        space = ANCHOR_DISCRIMINATOR + AccessReceipt::INIT_SPACE,
        seeds = [RECEIPT_SEED, buyer.key().as_ref(), research_paper.key().as_ref()],
        bump
    )]
    pub access_receipt: Account<'info, AccessReceipt>,

    pub system_program: Program<'info, System>,
}

impl<'a> PurchaseAccess<'a> {
    pub fn purchase_access(&mut self, bumps: &PurchaseAccessBumps) -> Result<()> {
        require!(
            self.research_paper.price > 0,
            ErrorCodes::ResearchPriceInvalid
        );
        require!(
            self.buyer.key() != self.research_paper.author,
            ErrorCodes::AuthorCantBuySelf
        );
        require!(
            self.buyer.lamports() >= self.research_paper.price,
            ErrorCodes::InsufficientFundsInWallet
        );

        //calculating the platform fee and author amount
        let total_amount = self.research_paper.price;
        let platform_fee = total_amount
            .checked_mul(GLOBAL_FEE_PERCENTAGE) // e.g., 100 * 5 = 500
            .ok_or(ErrorCodes::MathOverflow)?
            .checked_div(100) // e.g., 500 / 100 = 5
            .ok_or(ErrorCodes::MathOverflow)?;
        let author_earning = total_amount
            .checked_sub(platform_fee)
            .ok_or(ErrorCodes::MathOverflow)?;

        //transferring the author amount to the author vault
        let cpi_program = self.system_program.to_account_info();
        let buyer = self.buyer.to_account_info();
        let author_vault = self.author_vault.to_account_info();
        let cpi_account_options_author = Transfer {
            from: buyer,
            to: author_vault,
        };
        let cpi_ctx_author = CpiContext::new(cpi_program, cpi_account_options_author);
        transfer(cpi_ctx_author, author_earning)?;

        //transferring the platform fee to the admin vault
        let cpi_program = self.system_program.to_account_info();
        let buyer = self.buyer.to_account_info();
        let admin_vault = self.admin_vault.to_account_info();
        let cpi_account_options_admin = Transfer {
            from: buyer.to_account_info(),
            to: admin_vault,
        };
        let cpi_ctx_admin = CpiContext::new(cpi_program, cpi_account_options_admin);
        transfer(cpi_ctx_admin, platform_fee)?;

        //storing the receipt
        let buyer = self.buyer.key();
        let purchased_paper = self.research_paper.key();
        self.access_receipt.set_inner(AccessReceipt {
            buyer,
            purchased_paper,
            timestamp: Clock::get()?.unix_timestamp,
            bump: bumps.access_receipt,
        });
        //updating the states
        self.buyer_user_account
            .purchased
            .checked_add(1u16)
            .ok_or(ErrorCodes::MathOverflow)?;
        self.research_paper
            .sales
            .checked_add(1u32)
            .ok_or(ErrorCodes::MathOverflow)?;
        self.author_user_account
            .earning
            .checked_add(author_earning)
            .ok_or(ErrorCodes::MathOverflow)?;
        self.author_user_account
            .sold
            .checked_add(1u16)
            .ok_or(ErrorCodes::MathOverflow)?;

        Ok(())
    }
}

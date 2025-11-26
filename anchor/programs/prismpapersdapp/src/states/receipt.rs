use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AccessReceipt {
    pub buyer: Pubkey,
    pub purchased_paper: Pubkey,
    pub timestamp: i64,
    pub bump: u8,
}

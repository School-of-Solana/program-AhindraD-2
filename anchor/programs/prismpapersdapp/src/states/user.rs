use anchor_lang::prelude::*;

use crate::constants::USER_NAME_MAX_LENGTH;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub owner: Pubkey,
    #[max_len(USER_NAME_MAX_LENGTH)]
    pub name: String,
    pub published: u16,
    pub purchased: u16,
    pub sold: u16,
    pub reviewed: u16,
    pub earning: u64,
    pub timestamp: i64,
    pub bump: u8,
}

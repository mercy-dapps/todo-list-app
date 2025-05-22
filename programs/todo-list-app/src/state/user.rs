use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub author: Pubkey,
    pub todo_count: u8,
    pub bump: u8
}

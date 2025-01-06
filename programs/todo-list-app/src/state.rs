use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Task {
    pub author: Pubkey,
    pub  completed: bool,
    pub todo_id: u8,
    #[max_len(200)]
    pub  title: String,
}

#[account]
#[derive(InitSpace)]
pub struct User {
    pub author: Pubkey,
    pub last_todo: u8,
    pub todo_count: u8,
}

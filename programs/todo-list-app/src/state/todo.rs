use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Todo {
    pub author: Pubkey,
    pub completed: bool,
    pub todo_id: u8,
    #[max_len(100)]
    pub  title: String,
    pub bump: u8
}
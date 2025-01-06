use anchor_lang::prelude::*;

use crate::{constant::*, state::*};

pub fn _delete_task(ctx: Context<DeleteTask>, _todo_id: u8) -> Result<()> {
    let user = &mut ctx.accounts.user;

    user.todo_count = user.todo_count.checked_sub(1).unwrap();
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(todo_id: u8)]
pub struct DeleteTask<'info> {
    #[account(
        mut,
        has_one = author,
        seeds = [USER_TAG, author.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    #[account(
        mut,
        has_one = author,
        close = author,
        seeds = [TASK_TAG, author.key().as_ref(), &todo_id.to_le_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}
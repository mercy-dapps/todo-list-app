use anchor_lang::prelude::*;

use crate::{constant::*, state::*, error::*};

pub fn _delete_task(ctx: Context<DeleteTask>, todo_id: u8) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let task = &mut ctx.accounts.task;
    let author = &mut ctx.accounts.author;

    require!(task.author == author.key(), TodoError::Unauthorized);
    require!(task.todo_id == todo_id, TodoError::Unauthorized);

    user.last_todo = user.last_todo.checked_sub(1).unwrap();
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
        seeds = [TASK_TAG, author.key().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}
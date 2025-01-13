use anchor_lang::prelude::*;

use crate::{constant::*, state::*, error::*};

pub fn _edit_task(ctx: Context<EditTask>, todo_id: u8, new_title: String) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let author = &mut ctx.accounts.author;

    require!(task.author == author.key(), TodoError::Unauthorized);
    require!(task.todo_id == todo_id, TodoError::Unauthorized);

    task.title = new_title;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(todo_id: u8)]
pub struct EditTask<'info> {
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
        seeds = [TASK_TAG, author.key().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}
use anchor_lang::prelude::*;

use crate::{constant::*, error::*, state::*};

pub fn _add_task(ctx: Context<AddTask>, title: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let task = &mut ctx.accounts.task;

    require!(title.chars().count() <= 200, TodoError::TitleTooLong);

    task.author = ctx.accounts.author.key();
    task.todo_id = user.last_todo;
    task.title = title;
    task.completed = false;

    user.last_todo = user.last_todo.checked_add(1).unwrap();
    user.todo_count = user.todo_count.checked_add(1).unwrap();

    Ok(())
}

#[derive(Accounts)]
pub struct AddTask<'info> {
    #[account(
        mut,
        has_one = author,
        seeds = [USER_TAG, author.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    #[account(
        init,
        payer = author,
        space = Task::INIT_SPACE,
        seeds = [TASK_TAG, author.key().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}
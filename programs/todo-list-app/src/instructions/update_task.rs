use anchor_lang::prelude::*;

use crate::{constant::*, state::*};

pub fn _update_task(ctx: Context<UpdateTask>, _todo_id: u8) -> Result<()> {
    let task = &mut ctx.accounts.task;

    task.completed = !task.completed;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(todo_id: u8)]
pub struct UpdateTask<'info> {
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
        seeds = [TASK_TAG, author.key().as_ref(), &todo_id.to_le_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}
use anchor_lang::prelude::*;

mod error;

use crate::error::*;

declare_id!("4anCuMniXh6YLNBajw1Mwypf82eRtwqdPB5RnVBwwoLR");

const DISCRIMINATOR : usize = 8;
const PUBLIC_KEY_LENGTH : usize = 32;
const BOOL_LENGTH : usize = 1;
const TEXT_LENGTH : usize = 4 + 100 * 4;
const TIMESTAMP_LENGTH : usize = 8;

#[program]
pub mod todo_list_app {
    use super::*;

    pub fn adding_task(ctx: Context<AddingTask>, title: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();
        if title.chars().count() > 400 {
            return Err(TodoListError::TextTooLong.into());
        }

        task.author = *author.key;
        task.completed = false;
        task.created_at = clock.unix_timestamp;
        task.updated_at = clock.unix_timestamp;
        task.title = title;
        
        Ok(())
    }

    pub fn updating_task(ctx: Context<UpdatingTask>, completed: bool) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

        task.author = *author.key;
        task.completed = completed;
        task.updated_at = clock.unix_timestamp;

        Ok(())
    }

    pub fn deleting_task(ctx: Context<DeletingTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

        task.author = *author.key;
        task.completed = true;
        task.updated_at = clock.unix_timestamp;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct AddingTask<'info> {
    #[account(
        init,
        payer = author,
        space = Task::LEN
    )]
    pub task : Account<'info, Task>,

    #[account(mut)]
    pub author: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[account]
pub struct Task {
    pub author: Pubkey,
    pub  completed: bool,
    pub  title: String,
    pub  created_at: i64,
    pub updated_at: i64
}

impl Task {
    const LEN: usize = DISCRIMINATOR + PUBLIC_KEY_LENGTH + BOOL_LENGTH + TEXT_LENGTH + TIMESTAMP_LENGTH + TIMESTAMP_LENGTH;
}

#[derive(Accounts)]
pub struct UpdatingTask<'info> {
    #[account(
        mut,
        has_one = author
    )]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>
}

#[derive(Accounts)]
pub struct DeletingTask<'info> {
    #[account(
        mut,
        has_one = author
    )]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>
}
use anchor_lang::prelude::*;

use crate::{constant::*, error::*, state::*};

#[derive(Accounts)]
#[instruction(todo_id: u8)]
pub struct AddTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, author.key().as_ref()],
        bump = user.bump
    )]
    pub user: Account<'info, User>,

    #[account(
        init,
        payer = author,
        space = DISCRIMINATOR + Todo::INIT_SPACE,
        seeds = [TODO_TAG, author.key().as_ref(), &[todo_id]],
        bump,
    )]
    pub todo: Account<'info, Todo>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddTodo<'info>  {
    pub fn add_todo(&mut self, todo_id: u8, title: String,  bumps: &AddTodoBumps) -> Result<()> {
        require!(title.chars().count() <= 100, TodoError::TitleTooLong);

        self.todo.set_inner(Todo { 
            author: self.author.key(), 
            completed: false, 
            todo_id, 
            title, 
            bump: bumps.todo
        });

        self.user.set_inner(User { 
            author: self.author.key(), 
            todo_count: self.user.todo_count.checked_add(1).unwrap(), 
            bump: self.user.bump 
        });
    
        Ok(())
    } 
}
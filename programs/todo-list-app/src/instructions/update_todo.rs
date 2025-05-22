use anchor_lang::prelude::*;

use crate::{constant::*, state::*, error::*};

#[derive(Accounts)]
#[instruction(todo_id: u8)]
pub struct UpdateTodo<'info> {
    #[account(
        mut,
        has_one = author,
        seeds = [USER_TAG, author.key().as_ref()],
        bump = user.bump
    )]
    pub user: Account<'info, User>,

    #[account(
        mut,
        has_one = author,
        seeds = [TODO_TAG, author.key().as_ref(), &[todo_id]],
        bump = todo.bump
    )]
    pub todo: Account<'info, Todo>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateTodo<'info> {
    pub fn update_todo(&mut self, todo_id: u8) -> Result<()> {

        require!(self.todo.author == self.author.key(), TodoError::Unauthorized);
        require!(self.todo.todo_id == todo_id, TodoError::InvalidTodo);

        self.todo.set_inner(Todo { 
            author: self.author.key(), 
            completed: !self.todo.completed, 
            todo_id, 
            title: self.todo.title.clone(), 
            bump: self.todo.bump
        });
        
        Ok(())
    }
}
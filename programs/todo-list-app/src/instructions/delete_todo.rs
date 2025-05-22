use anchor_lang::prelude::*;

use crate::{constant::*, state::*, error::*};

#[derive(Accounts)]
#[instruction(todo_id: u8)]
pub struct DeleteTodo<'info> {
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
        close = author,
        seeds = [TODO_TAG, author.key().as_ref(), &[todo_id]],
        bump = todo.bump
    )]
    pub todo: Account<'info, Todo>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> DeleteTodo<'info>  {
    pub fn delete_todo(&mut self, todo_id: u8) -> Result<()> {
    
        require!(self.todo.author == self.author.key(), TodoError::Unauthorized);
        require!(self.todo.todo_id == todo_id, TodoError::InvalidTodo);

        self.user.set_inner(User { 
            author: self.author.key(), 
            todo_count: self.user.todo_count.checked_sub(1).unwrap(), 
            bump: self.todo.bump 
        });
        
        Ok(())
    }
}
pub mod constant;
pub mod state;
pub mod error;

pub mod instructions;

use anchor_lang::prelude::*;

use crate::instructions::*;

declare_id!("4anCuMniXh6YLNBajw1Mwypf82eRtwqdPB5RnVBwwoLR");

#[program]
pub mod todo_list_app {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn add_todo(ctx: Context<AddTodo>, todo_id: u8, title: String ) -> Result<()> {
        ctx.accounts.add_todo(todo_id, title, &ctx.bumps)
    }

    pub fn update_todo(ctx: Context<UpdateTodo>, todo_id: u8) -> Result<()> {
        ctx.accounts.update_todo(todo_id)
    }

    pub fn edit_todo(ctx: Context<EditTodo>, todo_id: u8, new_title: String) -> Result<()> {
        ctx.accounts.edit_todo(todo_id, new_title)
    }

    pub fn delete_todo(ctx: Context<DeleteTodo>, todo_id: u8) -> Result<()> {
        ctx.accounts.delete_todo(todo_id)
    }

}
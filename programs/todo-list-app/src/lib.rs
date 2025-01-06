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
        _initialize_user(ctx)
    }

    pub fn add_task(ctx: Context<AddTask>, title: String) -> Result<()> {
        _add_task(ctx, title)
    }

    pub fn update_task(ctx: Context<UpdateTask>, _todo_id: u8) -> Result<()> {
        _update_task(ctx, _todo_id)
    }

    pub fn delete_task(ctx: Context<DeleteTask>, _todo_id: u8) -> Result<()> {
        _delete_task(ctx, _todo_id)
    }

}
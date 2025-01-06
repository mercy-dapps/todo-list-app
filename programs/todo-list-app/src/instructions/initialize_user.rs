use anchor_lang::prelude::*;

use crate::{constant::*, state::*};

pub fn _initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
    let user = &mut ctx.accounts.user;

    user.author = ctx.accounts.author.key();
    user.last_todo = 0;
    user.todo_count = 0;

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(
        init,
        space = DISCRIMINATOR + User::INIT_SPACE,
        payer = author,
        seeds = [USER_TAG, author.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}
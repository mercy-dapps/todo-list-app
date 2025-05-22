use anchor_lang::prelude::*;

use crate::{constant::*, state::*};

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(
        init,
        space = DISCRIMINATOR + User::INIT_SPACE,
        payer = author,
        seeds = [USER_TAG, author.key().as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info>  {
    pub fn initialize_user(&mut self, bumps: &InitializeUserBumps) -> Result<()> {
        self.user.set_inner(User { 
            author: self.author.key(), 
            todo_count: 0, 
            bump: bumps.user
        });

        Ok(())
    }
}
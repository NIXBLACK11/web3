use anchor_lang::prelude::*;

pub mod constant;
pub mod states;
pub mod error;

/* We can destructure this states::VAR;
so we can write
*/
use crate::{constant::*, states::*, error::*};

declare_id!("6G445Uv3oFhehzywo3bW2S1far3GWLhEPhVGAKfgAhb1");

    // Initialize User
    // Add a USER profile to the blockchain
    // Add values for the default data


    // Add a TODO

    // Mark a TODO

    // Remove the TODO(deletion not possible)

#[program]
pub mod clever_todo {
    use super::*; //This brings all the crate inside this 

    pub fn initialize_user(
        ctx: Context<InitializeUser>
    ) -> Result<()> {
        // Initialize user profile with default data
        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.todo_count = 0;
        user_profile.last_todo = 0;
        user_profile.authority = ctx.accounts.authority.key();

        Ok(())
    }

    pub fn add_todo(
        ctx: Context<AddTodo>,
        content: String
    ) -> Result<()> {
        // Fill the TODO struct with the proper values
        let todo_account = &mut ctx.accounts.todo_account;
        let user_profile = &mut ctx.accounts.user_profile;

        todo_account.authority = ctx.accounts.authority.key();
        todo_account.idx = user_profile.last_todo;
        todo_account.content = content;
        todo_account.marked = false;

        user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();
        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn mark_todo(
        ctx: Context<MarkTodo>,
        todo_idx: u8
    ) -> Result<()> {
        // Mark the todo as done
        let todo_account  = &mut ctx.accounts.todo_account;
        require!(!todo_account.marked, TodoError::AlreadyMarked);
        
        todo_account.marked = true;

        Ok(())
    }

    pub fn remove_todo(
        ctx: Context<RemoveTodo>,
        todo_idx: u8
    ) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.todo_count = user_profile.todo_count.checked_sub(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction()]
pub struct AddTodo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [TODO_TAG, authority.key().as_ref(), &[user_profile.last_todo as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<TodoAccount>() + 8,
    )]
    pub todo_account:Box<Account<'info, TodoAccount>>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct MarkTodo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct RemoveTodo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        close = authority,
        seeds = [TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    pub system_program: Program<'info, System>
}
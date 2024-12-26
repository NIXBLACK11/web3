use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod constant;

use crate::{error::*, state::*, constant::*};

declare_id!("J9JiB2NCsRBUm5fY78rczXA7tcZxokuvLG1kpfjbQiZt");

#[program]
pub mod polling_app {
    use super::*;

    pub fn initialize_user(
        ctx: Context<InitializeUser>
    ) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.authority = ctx.accounts.authority.key();
        user_profile.total_polls = 0;
        
        Ok(())
    }

    pub fn make_poll(
        ctx: Context<MakePoll>,
        description: String,
        option1: String,
        option2: String,
        end_time: i64
    ) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let poll_account = &mut ctx.accounts.poll_account;

        poll_account.authority = ctx.accounts.authority.key();
        poll_account.idx = user_profile.total_polls;
        poll_account.description = description;
        poll_account.option1 = option1;
        poll_account.option1_count = 0;
        poll_account.option2 = option2;
        poll_account.option2_count = 0;
        poll_account.winner = "None".to_string();
        poll_account.end_time = end_time;

        user_profile.total_polls = user_profile.total_polls.checked_add(1).unwrap();

        Ok(())
    }

    pub fn select_option(
        ctx:Context<SelectOption>,
        poll_idx: u8,
        option: u8,
    ) -> Result<()> {
        let poll_account = &mut ctx.accounts.poll_account;
        let user_key = ctx.accounts.authority.key();

        if poll_account.voters.contains(&user_key) {
            return Err(PollError::AlreadyVoted.into());
        }

        match option {
            1 => poll_account.option1_count = poll_account
                .option1_count
                .checked_add(1)
                .ok_or(PollError::MathOverflow)?,
            2 => poll_account.option2_count = poll_account
                .option2_count
                .checked_add(1)
                .ok_or(PollError::MathOverflow)?,
            _ => return Err(PollError::NotAllowed.into()),
        }

        poll_account.voters.push(user_key);
        
        Ok(())
    }

    pub fn remove_poll(
        ctx: Context<RemovePoll>,
        poll_idx: u8,
    ) -> Result<()> {
        let poll_account = &mut ctx.accounts.poll_account;

        if poll_account.authority != ctx.accounts.authority.key() {
            return Err(PollError::Unauthorized.into());
        }
        
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
pub struct MakePoll<'info> {
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
        seeds = [POLL_TAG, authority.key().as_ref(), &[user_profile.total_polls as u8].as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<PollAccount>(),
    )]
    pub poll_account: Box<Account<'info, PollAccount>>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(poll_idx: u8)]
pub struct SelectOption<'info> {
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
        seeds = [POLL_TAG, authority.key().as_ref(), &[poll_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub poll_account: Box<Account<'info, PollAccount>>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(poll_idx: u8)]
pub struct RemovePoll<'info> {
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
        seeds = [POLL_TAG, authority.key().as_ref(), &[poll_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub poll_account: Box<Account<'info, PollAccount>>,

    pub system_program: Program<'info, System>
}
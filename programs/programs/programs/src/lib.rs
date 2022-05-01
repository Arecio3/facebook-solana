// Smart Contracts

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// limit Post to 1024 characters
const TEXT_LENGTH: usize = 1204;
const USER_NAME_LENGTH: usize = 100;
const USER_URL_LENGTH: usize = 255;

#[program]
pub mod programs {
    // makes scope global
    use super::*;
    // This gets passed in from frontend (be able to create/access state whenever)
    pub fn create_state(
        ctx: Context<CreateState>,
        // Whatever gets returned gets named Result
    ) -> Result<()> {
        // grab current state and make it mutable
        let state = &mut ctx.accounts.state;
        // Who ever is logged in gets authority
        state.authority = ctx.accounts.authority.key();

        state.post_count = 0;
        // Error Check
        Ok(())
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        text: String,
        account_name: String,
        account_url: String,
    )   -> Result<()> {
        
        let state = &mut ctx.accounts.state;
        
        let post = &mut ctx.accounts.post;

        post.authority = ctx.accounts.authority.key();

        post.text
        // Error Check
        Ok(())
    }
}

//  Defining Data Structures

#[derive(Accounts)]
// info has to do with lifetime in rust and anchor
pub struct CreateState<'info> {
    // Passing values into the state
    #[account(
        init,
        // Generates random seed everytime state variable is created
        seeds = [b"state".as_ref()],
        // algorithmn that makes sure seeds are always unique
        bump,
        payer = authority,
        space = size_of::<StateAccount>() + 8
    )]
    pub state: Account<'info, StateAccount>,

    // Authority (Who paid the transaction fee) mut = mutable meaning you can change authority
    #[account(mut)]
    pub authority: Signer<'info>,

    // System Program
    pub system_program: UncheckedAccount<'info>,

    // Token Program
    pub token_program: Program<'info, Token>
}

// Create Post
#[derive(Accounts)]
pub struct CreatePost<'info> {
    // Authenticates state account 
    #[account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, StateAccount>,

    // Authenticate Post Account
    #[account(
        init,
        // use "post" and index of post as a seed
        seeds = [b"post".as_ref(), state.post_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        // Type post account and give it a size that can fit all of its info
        space = size_of::<PostAccount>() + USER_URL_LENGTH + TEXT_LENGTH + USER_NAME_LENGTH
    )]
    pub post: Account<'info, PostAccount>

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: UncheckedAccount<'info>,

    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>

    // Time stamp
    pub clock: Sysvar<'info, Clock>,
}


// State Account
#[account]
pub struct StateAccount {
    pub authority: Pubkey,
    // unsigned iteger 64 bit (number)
    pub post_count: u64,
}

//  Post Account Structure (defining shape of object)
#[account] // What account made this post 
pub struct PostAccount {
    // Signer Address
    pub authority: Pubkey

    pub text: String,
    pub account_name: String,
    pub account_url: Srting,

    pub comment_count: u64,
    pub index: u64,

    pub post_time: i64,
}

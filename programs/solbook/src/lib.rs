use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use solana_program::system_instruction;

pub mod constant;
pub mod model;

use crate::{constant::*, model::*};

declare_id!("BgjKF9nf4SyUpqWcB5Y8go9dumRRR3ZsuhffcXGxbAMu");

#[program]
mod solbook {

    use super::*;

    pub fn initialize(ctx: Context<InitializeUser>, name : String, avatar : String, joined_at : String) -> Result<()> {

        let user_account = &mut ctx.accounts.user_account;
        let signer = &mut ctx.accounts.signer;

        user_account.name = name;
        user_account.avatar = avatar;
        user_account.joined_at = joined_at;
        user_account.level = 0;
        user_account.book_count = 0;
        user_account.last_book_id = 0;
        user_account.signer = signer.key();
        msg!("{}, You have successfully created a profile!", user_account.name); 
        Ok(())
    }

    pub fn get_writer_level(ctx : Context<InitializeUser>,) -> Result<()> {

        let user_account = &mut ctx.accounts.user_account;

        let level = user_account.level;

        msg!("{}", level);
        
        Ok(())
    }



    pub fn write_book(ctx:Context<WriteBook>, title : String, tag : Vec<String>, body : String) -> Result<()>{
        let book_account= &mut ctx.accounts.book_account;
        let user_account = &mut ctx.accounts.user_account;
        let signer = &mut ctx.accounts.signer;

        book_account.id = user_account.last_book_id;
        book_account.title = title;
        book_account.tag = tag;
        book_account.body = body;
        book_account.user = user_account.key();
        book_account.signer = signer.key();

        // 
        user_account.last_book_id = user_account.last_book_id.checked_add(1).unwrap();
        user_account.book_count = user_account.book_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn transfer_lamports(ctx: Context<TransferLamports>, amount: u64) -> Result<()> {
        let from_account = &ctx.accounts.from;
        let to_account = &ctx.accounts.to;

        // Create the transfer instruction
        let transfer_instruction =
            system_instruction::transfer(from_account.key, to_account.key, amount);

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                from_account.to_account_info(),
                to_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        Ok(())
    }

    
    pub fn initializestatepda(_ctx: Context<Initialisedstatepda>, _bump: u8) -> Result<()> {
        msg!("state got Initialised");

        Ok(())
    }

    pub fn initialisetokenpda(ctx: Context<Initialisetokenpda>, _bump1: u8) -> Result<()> {
        msg!("token got Initialised");
        let pda = ctx.accounts.tokenpda.key();
        msg!("token pda : {}", pda);
        Ok(())
    }

    pub fn sendtokenpda(
        ctx: Context<SendTokenPDA>,
        _bump1: u8,
        _bump2: u8,
        _amount: u64,
    ) -> Result<()> {
        msg!("token process start for PDA transfer...");
        let state = &mut (ctx.accounts.statepda);
        msg!("before: {}", state.amount);
        msg!("{} bump after", state.bump);
        state.bump = _bump1;
        state.amount = _amount;
        msg!("after: {}", state.amount);
        msg!("{} bump after", state.bump);
        let bump_vector = _bump1.to_le_bytes();
        let dep = &mut ctx.accounts.deposit_token_account.key();
        let sender = &ctx.accounts.owner;
        let inner = vec![
            sender.key.as_ref(),
            dep.as_ref(),
            "state".as_ref(),
            bump_vector.as_ref(),
        ];
        let outer = vec![inner.as_slice()];

        let transfer_instruction = Transfer {
            from: ctx.accounts.deposit_token_account.to_account_info(),
            to: ctx.accounts.tokenpda.to_account_info(),
            authority: sender.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        msg!("transfer call start");

        anchor_spl::token::transfer(cpi_ctx, _amount)?;
        ctx.accounts.tokenpda.reload()?;
        msg!("token pda key {}", ctx.accounts.tokenpda.key());
        msg!(
            "token after transfer to reciever in PDA {}",
            ctx.accounts.tokenpda.amount
        );

        msg!("succesfully transfered");

        Ok(())
    }
}

    

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(init, seeds = [USER_SEED, signer.key().as_ref()], bump, payer = signer, space = 8 + 260 + 2060 + 2 + 260 + 32)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction()]
pub struct WriteBook<'info> {
    #[account( init, seeds= [BOOK_SEED, signer.key().as_ref(), &[user_account.last_book_id]], bump, payer = signer, space = 8 + 260 + 2052 + 32 + 32)]
    pub book_account : Account<'info, BookState>,
    #[account(mut, seeds = [USER_SEED, signer.key().as_ref()], bump, has_one = signer)]
    pub user_account : Account<'info, UserAccount>,

    #[account(mut)]
    pub signer : Signer<'info>,
    pub system_program : Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(_bump : u8)]
pub struct Initialisedstatepda<'info> {
    #[account(
        init,
        payer = owner,
        seeds=[owner.key.as_ref(),deposit_token_account.key().as_ref(),"TokenState".as_ref()],
        bump,
        space=200
    )]
    statepda: Account<'info, TokenState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_bump : u8)]
pub struct Initialisetokenpda<'info> {
    #[account(
        init,
        seeds = [owner.key.as_ref(),deposit_token_account.key().as_ref()],
        bump,
        payer = owner,
        token::mint = mint,
        token::authority = statepda,
     )]
    pub tokenpda: Account<'info, TokenAccount>,
    pub statepda: Account<'info, TokenState>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}
#[derive(Accounts)]
pub struct SendTokenPDA<'info> {
    #[account(mut)]
    pub tokenpda: Account<'info, TokenAccount>,
    pub statepda: Account<'info, TokenState>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct UserAccount {
    name: String,
    avatar : String,
    level : u8,
    book_count : u8,
    last_book_id : u8,
    joined_at : String,
    signer : Pubkey,
}

#[account]
pub struct BookState {
    id : u8,
    title : String,
    tag : Vec<String>,
    body : String,
    user : Pubkey,
    signer : Pubkey
}

#[account]
#[derive(Default)]
pub struct TokenState {
   pub bump: u8,
   pub amount: u64,           
}



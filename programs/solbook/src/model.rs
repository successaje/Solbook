use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserAccount {
    name: String,
    avatar : String,
    level : u8,
    book_count : u8,
    last_book_id :u8,
    joined_at : String,
    signer : Pubkey,
}

#[account]
#[derive(Default)]
pub struct BookState {
    title : String,
    tag : Vec<String>,
    body : String,
    user : Pubkey,
    signer : Pubkey
}
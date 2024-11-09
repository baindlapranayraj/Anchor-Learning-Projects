use anchor_lang::prelude::*;

declare_id!("FsmpV94A6sFDuJpvwGAbydVj8zT1LGpNRSx3YSYBfrvR");

#[program]
pub mod counter_program {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter account is created");
        msg!("Current counter is {}", counter.count);
        Ok(())
    }
    pub fn increament(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter Variable is updated");
        msg!("Current Counter is: {}", counter.count);
        Ok(())
    }
    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_sub(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = DISCRIMINATOR + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>, // "Account":This is a solana account type,stores custoum data,for example now it is storeing "Counter" Data hear.We can create and store data of Counter.
    #[account(mut)]
    pub user: Signer<'info>, // "Signer":This type indicates that the trx's must be signed by this "user" account private key proving the ownership
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    // The "Account<'info, Counter>" type here is used to access the existing account data of type Counter
    // read and update its contents.
    pub user: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    count: u64,
}

const DISCRIMINATOR: usize = 8;

//=================================== Learnings ===================================
// - #[account(init, ...)]: This attribute indicates that this account will be initialized
//  (i.e., created) when the transaction is executed and adds metadata for how the account should be treated.
//
// - The #[account(init, ...)] attribute is used when you want to create a new account on the Solana blockchain.
//
// - payer = user: Specifies that the user account will pay for the creation of the "counter" account.
// This means the rent and transaction fees will be deducted from the user's wallet.
//
// - #[account(mut)]: This macro indicates that this particular account(user) is mutable,meaning it can
//  modify within the trx's.
//
// - #[derive(Accounts)]:This attribute tells the Anchor framework that Initialize is a collection of
//   accounts that will be passed into your initialize function.
//
// - The #[account(mut)] attribute simply allows you to modify the data within the already existing account
//  with some trasaction fees for updating the account.

// - Account<'info, T> is used for both creating and accessing accounts:
//  -- With init attribute: It creates a new account with type T.
//  -- Without init attribute: It accesses an existing account with type T.

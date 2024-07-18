use anchor_lang::prelude::*;

declare_id!("GJtgLhR2t7CPrh6hmYpQDr4aypeq4yqViQxn9TD2BR6i");

#[program]
pub mod sc_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

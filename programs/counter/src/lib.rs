use anchor_lang::prelude::*;

declare_id!("8JHjZNvPYWjfiq3xb3zBorXydcdxedY9Rhc1ng2YuuDw");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

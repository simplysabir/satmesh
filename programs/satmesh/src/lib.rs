use anchor_lang::prelude::*;

declare_id!("9NDusP73MVbsD2tt8RFEP83jBRwXw3aw37PkeeCRbFxs");

#[program]
pub mod satmesh {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

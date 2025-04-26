#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;

declare_id!("FZJ5m8nT7mi78VGrGsCGSPRYSK69PS7U2rzvR3CwGcBP");

#[program]
pub mod fairflow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

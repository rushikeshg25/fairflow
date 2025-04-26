#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod states;
pub use instructions::*;
pub use states::*;

declare_id!("FZJ5m8nT7mi78VGrGsCGSPRYSK69PS7U2rzvR3CwGcBP");

#[program]
pub mod fairflow {
    use super::*;

    pub fn initialize_company_state(
        ctx: Context<Initialize>,
        name: String,
        inc_percent: u8,
        dec_percent: u8,
        treasury: Pubkey,
    ) -> Result<()> {
        ctx.accounts
            .init_company_state(name, inc_percent, dec_percent, treasury, &ctx.bumps)
    }
}

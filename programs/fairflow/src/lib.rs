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
        company_name: String,
        inc_percent: u8,
        dec_percent: u8,
        treasury: Pubkey,
    ) -> Result<()> {
        ctx.accounts.init_company_state(
            company_name,
            inc_percent,
            dec_percent,
            treasury,
            &ctx.bumps,
        )
    }

    pub fn create_team_state(
        ctx: Context<CreateTeam>,
        team_name: String,
        company_name: String,
    ) -> Result<()> {
        ctx.accounts
            .create_team_state(team_name, company_name, ctx.bumps)
    }

    pub fn register_employee(
        ctx: Context<RegisterEmployee>,
        team_name: String,
        company_name: String,
        salary_account: Pubkey,
        employee_name: String,
    ) -> Result<()> {
        ctx.accounts.register_employee(
            team_name,
            company_name,
            salary_account,
            employee_name,
            ctx.bumps,
        )
    }

    // pub fn process_payroll(ctx: Context<ProcessPayroll>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn submit_feedback(ctx: Context<SubmitFeedback>) -> Result<()> {
    //     Ok(())
    // }
}

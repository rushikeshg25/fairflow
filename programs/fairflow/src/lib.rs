#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod constants;
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

    pub fn submit_feedback(
        ctx: Context<SubmitFeedback>,
        feedback_for: Pubkey,
        team_name: String,
        company_name: String,
        feedback_rating: u8,
    ) -> Result<()> {
        ctx.accounts
            .submit_feedback(feedback_for, team_name, company_name, feedback_rating)
        //Prob might need bumps here
    }

    pub fn process_payroll(
        ctx: Context<ProcessPayroll>,
        team_name: String,
        company_name: String,
        salary_account: Pubkey,
        salary: u16,
        encrypted_salary: u16,
    ) -> Result<()> {
        ctx.accounts.process_payroll(
            team_name,
            company_name,
            salary_account,
            salary,
            encrypted_salary,
        )
    }
}

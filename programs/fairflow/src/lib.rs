#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;
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
    ) -> Result<()> {
        ctx.accounts
            .init_company_state(company_name, inc_percent, dec_percent, &ctx.bumps)
    }

    pub fn fund_treasury(
        ctx: Context<FundTreasury>,
        company_name: String,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.fund_treasury(company_name, amount)
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
        employee_name: String,
        employee_owned_salary_wallet: Pubkey,
        current_salary: u16,
        key: u16,
    ) -> Result<()> {
        ctx.accounts.register_employee(
            team_name,
            company_name,
            employee_name,
            employee_owned_salary_wallet,
            current_salary,
            key,
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
    }

    pub fn process_payroll(
        ctx: Context<ProcessPayroll>,
        team_name: String,
        company_name: String,
        employee_owned_salary_wallet: Pubkey,
        encryption_key: u16,
    ) -> Result<()> {
        ctx.accounts.process_payroll(
            team_name,
            company_name,
            employee_owned_salary_wallet,
            encryption_key,
        )
    }
}

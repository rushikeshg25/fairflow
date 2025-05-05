use anchor_lang::prelude::*;

use crate::{
    constants::ANCHOR_DISCRIMINATOR, errors::CompanyError, utils::encrypt_decrypt_salary, Employee,
    Team,
};

#[derive(Accounts)]
#[instruction(team_name: String, company_name: String,salary_account: Pubkey)]
pub struct RegisterEmployee<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,
    #[account(
        mut,
        seeds= [b"team",team_name.as_bytes(),company_name.as_bytes()],
        bump = team_state.bump,
    )]
    pub team_state: Account<'info, Team>,

    #[account(
        init,
        seeds= [b"employee",company_name.as_bytes(),salary_account.as_ref()],
        bump,
        payer = employer,
        space = ANCHOR_DISCRIMINATOR + Employee::INIT_SPACE,
    )]
    pub employee_state: Account<'info, Employee>,

    pub system_program: Program<'info, System>,
}

impl<'info> RegisterEmployee<'info> {
    pub fn register_employee(
        &mut self,
        _team_name: String,
        _company_name: String,
        salary_account: Pubkey,
        employee_name: String,
        current_salary: u16,
        key: u16,
        bumps: RegisterEmployeeBumps,
    ) -> Result<()> {
        require!(
            employee_name.len() > 0 && employee_name.len() <= 10,
            CompanyError::InvalidEmployeeName
        );
        self.employee_state.set_inner(Employee {
            employee_name: employee_name,
            team: self.team_state.key(),
            salary_account,
            last_payroll_feedback: 0,
            current_total_feedback_score: 0,
            current_total_feedbacks: 0,
            encrypted_current_salary: encrypt_decrypt_salary(key, current_salary),
            bump: bumps.employee_state,
        });

        self.team_state.employees.push(self.employee_state.key());

        Ok(())
    }
}

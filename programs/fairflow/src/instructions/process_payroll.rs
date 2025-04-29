use crate::constants::LAMPORTS_PER_SOL;
use crate::errors::CompanyError;
use crate::{Company, Employee};
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
#[instruction(team_name: String, company_name: String, salary_account: Pubkey)]
pub struct ProcessPayroll<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"employee", company_name.as_bytes(), salary_account.as_ref()],
        bump = employee_state.bump,
    )]
    pub employee_state: Account<'info, Employee>,

    #[account(
        seeds = [b"company", company_name.as_bytes(), employer.key().as_ref()],
        bump = company_state.bump,
    )]
    pub company_state: Account<'info, Company>,

    #[account(
        mut,
        address = employee_state.salary_account,
    )]
    pub salary_destination: SystemAccount<'info>,

    #[account(
        mut,
        address = company_state.treasury @ CompanyError::InvalidTreasury,
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ProcessPayroll<'info> {
    pub fn process_payroll(
        &mut self,
        _team_name: String,
        _company_name: String,
        _salary_account: Pubkey,
        salary: u16,
        encrypted_salary: u16,
    ) -> Result<()> {
        if self.employee_state.current_total_feedbacks == 0 {
        } else {
            self.employee_state.last_payroll_feedback =
                self.employee_state.current_total_feedback_score
                    / self.employee_state.current_total_feedbacks;
        }

        self.employee_state.encrypted_current_salary = encrypted_salary;
        self.employee_state.current_total_feedback_score = 0;
        self.employee_state.current_total_feedbacks = 0;

        if salary > 0 {
            let transfer_amount = (salary as u64) * LAMPORTS_PER_SOL;

            if self.treasury.lamports() < transfer_amount {
                return Err(error!(CompanyError::InsufficientFunds));
            }

            match system_program::transfer(
                CpiContext::new(
                    self.system_program.to_account_info(),
                    system_program::Transfer {
                        from: self.treasury.to_account_info(),
                        to: self.salary_destination.to_account_info(),
                    },
                ),
                transfer_amount,
            ) {
                Ok(_) => {}
                Err(e) => {
                    msg!("Failed to process salary payment: {:?}", e);
                    return Err(error!(CompanyError::PaymentTransferFailed));
                }
            }
        }

        Ok(())
    }
}

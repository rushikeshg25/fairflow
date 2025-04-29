use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Employee {
    #[max_len(10)]
    pub employee_name: String,
    pub team: Pubkey,
    pub salary_account: Pubkey,
    pub last_payroll_feedback: u8,
    pub current_total_feedback_score: u8,
    pub current_total_feedbacks: u8,
    pub encrypted_current_salary: u16,
    pub bump: u8,
}

//Salary will be encrypted off chain and then stored in this state

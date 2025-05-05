use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Employee {
    #[max_len(10)]
    pub employee_name: String,
    pub team: Pubkey,
    pub salary_account: Pubkey,
    pub employee_owned_salary_wallet: Pubkey,
    pub last_payroll_feedback: u8,
    pub current_total_feedback_score: u8,
    pub current_total_feedbacks: u8,
    pub encrypted_current_salary: u16,
    pub salary_account_bump: u8,
    pub bump: u8,
}

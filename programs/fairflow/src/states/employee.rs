use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Employee {
    #[max_len(10)]
    pub name: String,
    pub team: Pubkey,
    pub salary_account: Pubkey,
    pub last_payroll_feedback: u8,
    pub current_total_feedback_score: u8, //sum of current feedbacks
    pub current_total_feedbacks: u8,      //total num of teamates who have given feedback
    pub bump: u8,
}

//TODO: store Encrypted Salary

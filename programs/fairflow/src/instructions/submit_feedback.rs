use anchor_lang::prelude::*;

use crate::{errors::CompanyError, Employee, Team};

#[derive(Accounts)]
#[instruction(feedback_for: Pubkey,team_name: String, company_name: String)]
pub struct SubmitFeedback<'info> {
    #[account(mut)]
    pub employee_providing_feedback: Signer<'info>,
    #[account(
        mut,
        seeds= [b"team",team_name.as_bytes(),company_name.as_bytes()],
        bump = team_state.bump,
    )]
    pub team_state: Account<'info, Team>,

    #[account(
        mut,
        seeds= [b"employee",company_name.as_bytes(),feedback_for.as_ref()],
        bump = employee_to_feedback_state.bump,
    )]
    pub employee_to_feedback_state: Account<'info, Employee>,

    #[account(
        mut,
        seeds= [b"employee",company_name.as_bytes(),employee_providing_feedback.key().as_ref()],
        bump = employee_providing_feedback_state.bump,
    )]
    pub employee_providing_feedback_state: Account<'info, Employee>,

    pub system_program: Program<'info, System>,
}

impl<'info> SubmitFeedback<'info> {
    pub fn submit_feedback(
        &mut self,
        _feedback_for: Pubkey,
        _team_name: String,
        _company_name: String,
        feedback_rating: u8,
    ) -> Result<()> {
        require!(
            self.employee_providing_feedback_state.employee_name
                != self.employee_to_feedback_state.employee_name,
            CompanyError::CannotVoteForSelf
        );
        require!(
            self.employee_to_feedback_state.team == self.team_state.key(),
            CompanyError::EmployeeNotInTeam
        );
        require!(
            feedback_rating >= 1 && feedback_rating <= 5,
            CompanyError::InvalidFeedbackRating
        );

        self.employee_to_feedback_state.current_total_feedback_score += feedback_rating;
        self.employee_to_feedback_state.current_total_feedbacks += 1;
        Ok(())
    }
}

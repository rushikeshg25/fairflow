use anchor_lang::prelude::*;

use crate::{Employee, Team};

#[derive(Accounts)]
#[instruction(feedback_for: Pubkey, feedback_rating: u8, team_name: String, company_name: String)]
pub struct SubmitFeedback<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds= [b"team",team_name.as_bytes(),company_name.as_bytes()],
        bump = team_state.bump,
    )]
    pub team_state: Account<'info, Team>,

    #[account(
        mut,
        seeds= [b"employee",company_name.as_bytes(),feedback_for.as_ref()],
        bump = employee_state.bump,
    )]
    pub employee_state: Account<'info, Employee>,

    pub system_program: Program<'info, System>,
}

impl<'info> SubmitFeedback<'info> {
    pub fn submit_feedback(
        &mut self,
        _feedback_for: Pubkey,
        _feedback_rating: u8,
        _team_name: String,
        _company_name: String,
        _bumps: SubmitFeedbackBumps,
    ) -> Result<()> {
        Ok(())
    }
}

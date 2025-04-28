use anchor_lang::prelude::*;

use crate::{
    errors::CompanyError,
    states::{Company, Team},
};

#[derive(Accounts)]
#[instruction(team_name: String, company_name: String)]
pub struct CreateTeam<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,

    #[account(
        mut,
        seeds= [b"company",company_name.as_bytes(),employer.key().as_ref()],
        bump = company_state.bump,

    )]
    pub company_state: Account<'info, Company>,

    #[account(
        init,
        seeds= [b"team",team_name.as_bytes(),company_name.as_bytes()],
        bump,
        payer = employer,
        space = 8 + Team::INIT_SPACE,
    )]
    pub team_state: Account<'info, Team>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateTeam<'info> {
    pub fn create_team_state(
        &mut self,
        team_name: String,
        _company_name: String,
        bumps: CreateTeamBumps,
    ) -> Result<()> {
        require!(
            team_name.len() > 0 && team_name.len() <= 10,
            CompanyError::InvalidTeamName
        );
        require!(
            self.company_state.teams.len() < 5,
            CompanyError::MaxTeamsReached
        );
        self.team_state.set_inner(Team {
            team_name,
            employees: Vec::new(),
            bump: bumps.team_state,
        });
        self.company_state.teams.push(self.team_state.key());
        Ok(())
    }
}

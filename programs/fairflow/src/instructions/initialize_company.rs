use anchor_lang::prelude::*;

use crate::{constants::ANCHOR_DISCRIMINATOR, errors::CompanyError, states::Company};

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,
    #[account(
        init,
        payer = employer,
        space = ANCHOR_DISCRIMINATOR + Company::INIT_SPACE,
        seeds= [b"company",company_name.as_bytes(),employer.key().as_ref()],
        bump,
    )]
    pub company_state: Account<'info, Company>,

    #[account(
        seeds = [b"treasury", company_state.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init_company_state(
        &mut self,
        company_name: String,
        inc_percent: u8,
        dec_percent: u8,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        require!(
            company_name.len() > 0 && company_name.len() <= 10,
            CompanyError::InvalidCompanyName
        );
        self.company_state.set_inner(Company {
            company_name,
            treasury: self.treasury.key(),
            teams: Vec::new(),
            inc_percent,
            dec_percent,
            bump: bumps.company_state,
        });
        Ok(())
    }
}

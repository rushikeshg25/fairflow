use crate::{constants::LAMPORTS_PER_SOL, errors::CompanyError, states::Company};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct FundTreasury<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,
    #[account(
        seeds= [b"company",company_name.as_bytes(),employer.key().as_ref()],
        bump=company_state.bump,
    )]
    pub company_state: Account<'info, Company>,

    #[account(
        mut,
        seeds = [b"treasury", company_state.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> FundTreasury<'info> {
    pub fn fund_treasury(&mut self, _company_name: String, amount: u64) -> Result<()> {
        let required_lamports = amount
            .checked_mul(LAMPORTS_PER_SOL)
            .ok_or(CompanyError::ArithmeticOverflow)?;

        if self.employer.lamports() < required_lamports {
            msg!(
                "Insufficient employer balance. Has {}, needs {}",
                self.employer.lamports(),
                required_lamports
            );
            return Err(error!(CompanyError::InsufficientFunds));
        }

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.employer.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, required_lamports)?;

        Ok(())
    }
}

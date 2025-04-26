use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Company {
    #[max_len(10)]
    pub name: String,
    pub treasury: Pubkey,
    #[max_len(5)]
    pub teams: Vec<Pubkey>,
    pub inc_percent: u8,
    pub dec_percent: u8,
    pub bump: u8,
}

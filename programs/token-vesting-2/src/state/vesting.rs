use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vesting {
    pub authority: Pubkey,
    #[max_len(50)]
    pub company_name: String,
    pub token_mint: Pubkey,
    pub treasury_account: Pubkey,
    pub bump: u8,
}

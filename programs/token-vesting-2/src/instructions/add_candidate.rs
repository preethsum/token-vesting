use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct AddCandidate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub candidate: SystemAccount<'info>,
}

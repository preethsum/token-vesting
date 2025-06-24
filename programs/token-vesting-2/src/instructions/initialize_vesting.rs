use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Vesting, ANCHOR_DISCRIMINATION};

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct InitializeVesting<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATION + Vesting::INIT_SPACE,
        seeds = [b"vesting", authority.key().as_ref(), 
                 company_name.as_bytes()],
        bump
    )]
    pub vesting: Account<'info, Vesting>,

    // what is the difference between associated_token constraint and the token constraint?
    #[account(
        init,
        payer = authority,
        associated_token::mint = token_mint,
        associated_token::authority = vesting,
        associated_token::token_program = token_program,
    )]
    pub treasury_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::token_program = token_program,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn create_vesting_account(ctx: Context<InitializeVesting>, company_name: String) -> Result<()> {
    *ctx.accounts.vesting = Vesting {
        authority: ctx.accounts.authority.key(),
        company_name,
        token_mint: ctx.accounts.token_mint.key(),
        treasury_account: ctx.accounts.treasury_account.key(),
        bump: ctx.bumps.vesting,
    };
    Ok(())
}

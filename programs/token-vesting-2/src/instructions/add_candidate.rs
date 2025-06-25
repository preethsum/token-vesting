use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::{Candidate, Vesting, ANCHOR_DISCRIMINATION};

#[derive(Accounts)]
pub struct AddCandidate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub candidate: SystemAccount<'info>,

    #[account(
        mint::token_program = token_program,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = vesting,
        associated_token::token_program = token_program,
    )]
    pub treasury_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        has_one = authority,
        has_one = token_mint,
        seeds = [b"vesting", authority.key().as_ref(), vesting.company_name.as_bytes()],
        bump = vesting.bump,
    )]
    pub vesting: Account<'info, Vesting>,

    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATION + Candidate::INIT_SPACE,
        seeds = [b"candidate", candidate.key().as_ref(), vesting.key().as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, Candidate>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn transfer_tokens_to_treasury(ctx: &Context<AddCandidate>, amount: u64) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.authority.to_account_info(),
        to: ctx.accounts.treasury_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    transfer_checked(cpi_context, amount, ctx.accounts.token_mint.decimals)
}

pub fn create_candidate_account(
    ctx: Context<AddCandidate>,
    start_time: i64,
    end_time: i64,
    cliff_time: i64,
    amount: u64,
) -> Result<()> {
    *ctx.accounts.candidate_account = Candidate {
        candidate: ctx.accounts.candidate.key(),
        vesting: ctx.accounts.vesting.key(),
        total_amount: amount,
        start_time,
        end_time,
        cliff_time,
        withdrawn_amount: 0,
        bump: ctx.bumps.candidate_account,
    };
    Ok(())
}

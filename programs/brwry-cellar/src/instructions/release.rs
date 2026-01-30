use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use brwry_curves::{sample_curve, CurveKind, CurveParams, SCALE};

use crate::errors::BrwryError;
use crate::state::{Cask, CurveKindTag, Schedule};

#[derive(Accounts)]
pub struct ReleaseBarrel<'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,

    #[account(
        mut,
        seeds = [Cask::SEED, cask.authority.as_ref(), cask.recipient.as_ref(), cask.mint.as_ref()],
        bump = cask.bump,
        has_one = recipient @ BrwryError::UnauthorizedRecipient,
        has_one = mint,
        has_one = vault,
    )]
    pub cask: Account<'info, Cask>,

    #[account(
        mut,
        seeds = [Schedule::SEED, cask.key().as_ref()],
        bump = schedule.bump,
    )]
    pub schedule: Account<'info, Schedule>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, constraint = recipient_ata.owner == recipient.key())]
    pub recipient_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<ReleaseBarrel>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let cask = &mut ctx.accounts.cask;
    let schedule = &mut ctx.accounts.schedule;

    require!(now >= cask.cliff_ts, BrwryError::NothingToRelease);

    let claimable = compute_claimable(cask, now)?;
    require!(claimable > 0, BrwryError::NothingToRelease);

    let authority_key = cask.authority;
    let recipient_key = cask.recipient;
    let mint_key = cask.mint;
    let bump = cask.bump;
    let seeds: &[&[u8]] = &[
        Cask::SEED,
        authority_key.as_ref(),
        recipient_key.as_ref(),
        mint_key.as_ref(),
        core::slice::from_ref(&bump),
    ];
    let signer = &[seeds];

    let cpi = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient_ata.to_account_info(),
            authority: cask.to_account_info(),
        },
        signer,
    );
    transfer_checked(cpi, claimable, ctx.accounts.mint.decimals)?;

    cask.released_amount = cask


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

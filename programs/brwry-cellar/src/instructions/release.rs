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
        .released_amount
        .checked_add(claimable)
        .ok_or(BrwryError::MathOverflow)?;
    schedule.last_claim_ts = now;
    schedule.current_period = schedule.current_period.saturating_add(1);

    Ok(())
}

fn compute_claimable(cask: &Cask, now: i64) -> Result<u64> {
    let params = CurveParams {
        kind: match cask.curve {
            CurveKindTag::Linear => CurveKind::Linear,
            CurveKindTag::Cliff => CurveKind::Cliff,
            CurveKindTag::Exponential => CurveKind::Exponential,
            CurveKindTag::Logarithmic => CurveKind::Logarithmic,
            CurveKindTag::SCurve => CurveKind::SCurve,
        },
        cliff_at: cliff_scaled(cask),
        k_milli: cask.k_milli,
        steepness_milli: cask.steepness_milli,
    };

    let t = cask.progress_scaled(now);
    let fraction = sample_curve(params, t) as u128;
    let unlocked = (cask.total_amount as u128 * fraction) / SCALE as u128;
    let already = cask.released_amount as u128;
    if unlocked <= already {
        return Ok(0);
    }
    Ok((unlocked - already) as u64)
}

fn cliff_scaled(cask: &Cask) -> u64 {
    if cask.cliff_ts <= cask.start_ts {
        return 0;
    }
    if cask.cliff_ts >= cask.end_ts {
        return SCALE;
    }
    let numer = (cask.cliff_ts - cask.start_ts) as u128;
    let denom = (cask.end_ts - cask.start_ts) as u128;
    ((numer * SCALE as u128) / denom) as u64
}

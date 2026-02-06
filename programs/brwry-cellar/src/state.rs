use anchor_lang::prelude::*;

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveKindTag {
    Linear = 0,
    Cliff = 1,
    Exponential = 2,
    Logarithmic = 3,
    SCurve = 4,
}

#[account]
pub struct Cask {
    pub authority: Pubkey,
    pub recipient: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub total_amount: u64,
    pub released_amount: u64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub cliff_ts: i64,
    pub curve: CurveKindTag,
    pub k_milli: u64,
    pub steepness_milli: u64,
    pub bump: u8,
}

impl Cask {
    pub const SEED: &'static [u8] = b"cask";
    pub const SPACE: usize = 8 + 32 * 4 + 8 * 6 + 8 + 8 + 1 + 1 + 32;

    pub fn progress_scaled(&self, now: i64) -> u64 {
        if now <= self.start_ts {
            return 0;
        }
        if now >= self.end_ts {
            return brwry_curves::SCALE;
        }
        let elapsed = (now - self.start_ts) as u128;
        let total = (self.end_ts - self.start_ts) as u128;
        ((elapsed * brwry_curves::SCALE as u128) / total) as u64
    }
}

#[account]
pub struct Schedule {
    pub cask: Pubkey,
    pub periods: u32,

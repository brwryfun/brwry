#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod schedule;

pub use schedule::{sample_curve, sample_schedule, CurveKind, CurveParams};

pub const SCALE: u64 = 1_000_000;

#[inline]
pub fn clamp_scaled(t: u64) -> u64 {
    if t > SCALE {
        SCALE
    } else {
        t
    }
}

pub fn linear(t: u64) -> u64 {
    clamp_scaled(t)
}

pub fn cliff(t: u64, cliff_at: u64) -> u64 {
    let t = clamp_scaled(t);
    let c = clamp_scaled(cliff_at);
    if t <= c {
        return 0;
    }
    let numer = (t - c) as u128;
    let denom = (SCALE - c) as u128;
    ((numer * SCALE as u128) / denom) as u64
}

pub fn exponential(t: u64, k_milli: u64) -> u64 {
    let t = clamp_scaled(t);
    let scale = SCALE as u128;
    let k_scaled = k_milli as u128 * scale / 1000;
    let x = (k_scaled * t as u128) / scale;

    let numer = expm1_scaled(x);

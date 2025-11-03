#![cfg_attr(not(feature = "std"), no_std)]

pub const SCALE: u64 = 1_000_000;

pub fn linear(t: u64) -> u64 {
    t.min(SCALE)
}

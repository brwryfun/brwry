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
    let denom = expm1_scaled(k_scaled);
    if denom == 0 {
        return 0;
    }
    ((numer * scale) / denom) as u64
}

pub fn logarithmic(t: u64, k_milli: u64) -> u64 {
    let t = clamp_scaled(t);
    let scale = SCALE as u128;
    let k_scaled = k_milli as u128 * scale / 1000;
    let kt = (k_scaled * t as u128) / scale;

    let numer = log1p_scaled(kt);
    let denom = log1p_scaled(k_scaled);
    if denom == 0 {
        return 0;
    }
    ((numer * scale) / denom) as u64
}

pub fn s_curve(t: u64, steepness_milli: u64) -> u64 {
    let t = clamp_scaled(t);
    let scale = SCALE as i128;
    let s = steepness_milli as i128 * scale / 1000;
    let half = scale / 2;

    let arg_t = (s * (t as i128 - half)) / scale;
    let arg_0 = -s / 2;
    let arg_1 = s / 2;

    let raw_t = sigmoid_scaled(arg_t);
    let raw_0 = sigmoid_scaled(arg_0);
    let raw_1 = sigmoid_scaled(arg_1);

    if raw_1 == raw_0 {
        return 0;
    }
    let numer = raw_t - raw_0;
    let denom = raw_1 - raw_0;
    ((numer * scale) / denom).max(0).min(scale) as u64
}

fn expm1_scaled(x: u128) -> u128 {
    let scale = SCALE as u128;
    if x == 0 {
        return 0;
    }
    let mut term = x;
    let mut sum = x;
    let mut n: u128 = 2;
    while n <= 18 {
        term = (term * x) / (scale * n);
        if term == 0 {
            break;
        }
        sum += term;
        n += 1;
    }
    sum
}

fn log1p_scaled(x: u128) -> u128 {
    // log(1 + x) via atanh identity: log(1 + x) = 2 * atanh(x / (2 + x)).
    let scale = SCALE as u128;
    if x == 0 {
        return 0;
    }
    let z = (x * scale) / (2 * scale + x);
    let z2 = (z * z) / scale;
    let mut term = z;
    let mut sum = z;
    let mut n: u128 = 3;
    while n <= 25 {
        term = (term * z2) / scale;
        let add = term / n;
        if add == 0 {
            break;
        }
        sum += add;
        n += 2;
    }
    2 * sum
}

fn sigmoid_scaled(x: i128) -> i128 {
    // 1 / (1 + e^{-x}) on signed scale. Uses expm1_scaled on |x|.
    let scale = SCALE as i128;
    if x >= 0 {
        let ex_m1 = expm1_scaled(x as u128) as i128;
        let ex = ex_m1 + scale;
        (ex * scale) / (scale + ex)
    } else {
        let ex_m1 = expm1_scaled((-x) as u128) as i128;
        let ex_neg = (scale * scale) / (scale + ex_m1);
        (scale * scale) / (scale + ex_neg)
    }
}

#[cfg(test)]

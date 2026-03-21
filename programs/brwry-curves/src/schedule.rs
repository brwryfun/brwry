use alloc::vec::Vec;

use crate::{cliff, exponential, linear, logarithmic, s_curve, SCALE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveKind {
    Linear,
    Cliff,
    Exponential,
    Logarithmic,
    SCurve,
}

#[derive(Clone, Copy, Debug)]
pub struct CurveParams {
    pub kind: CurveKind,
    pub cliff_at: u64,
    pub k_milli: u64,
    pub steepness_milli: u64,
}

impl Default for CurveParams {
    fn default() -> Self {
        Self {
            kind: CurveKind::Linear,
            cliff_at: SCALE / 4,
            k_milli: 3_000,
            steepness_milli: 6_000,
        }
    }
}

pub fn sample_curve(params: CurveParams, t: u64) -> u64 {
    match params.kind {
        CurveKind::Linear => linear(t),
        CurveKind::Cliff => cliff(t, params.cliff_at),
        CurveKind::Exponential => exponential(t, params.k_milli),
        CurveKind::Logarithmic => logarithmic(t, params.k_milli),
        CurveKind::SCurve => s_curve(t, params.steepness_milli),
    }
}

pub fn sample_schedule(
    params: CurveParams,
    total_tokens: u128,
    start_unix: i64,
    end_unix: i64,
    periods: u32,
) -> Vec<(i64, u128)> {
    let mut out = Vec::with_capacity(periods as usize);
    if periods == 0 || end_unix <= start_unix {
        return out;
    }
    let duration = (end_unix - start_unix) as u128;
    let mut previous: u128 = 0;
    for i in 1..=periods {
        let t = ((i as u128) * SCALE as u128) / periods as u128;
        let frac = sample_curve(params, t as u64) as u128;
        let released = (total_tokens * frac) / SCALE as u128;
        let delta = released.saturating_sub(previous);
        previous = released;
        let ts = start_unix + ((i as u128 * duration) / periods as u128) as i64;
        out.push((ts, delta));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_periods_sum_to_total() {
        let params = CurveParams {
            kind: CurveKind::SCurve,
            ..Default::default()
        };
        let total: u128 = 1_000_000_000;
        let plan = sample_schedule(params, total, 0, 100, 10);
        let sum: u128 = plan.iter().map(|(_, amount)| *amount).sum();
        assert!(sum <= total);
        assert!(total - sum < 10);
    }

    #[test]
    fn zero_periods_returns_empty_plan() {
        let plan = sample_schedule(CurveParams::default(), 1_000, 0, 10, 0);
        assert!(plan.is_empty());
    }
}

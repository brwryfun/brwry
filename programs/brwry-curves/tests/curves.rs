use brwry_curves::{
    cliff, exponential, linear, logarithmic, s_curve, sample_schedule, CurveKind, CurveParams,
    SCALE,
};

const EPS: u64 = 2_000;

fn near(a: u64, b: u64) -> bool {
    let diff = if a > b { a - b } else { b - a };
    diff <= EPS
}

#[test]
fn linear_boundary_values() {
    assert_eq!(linear(0), 0);
    assert_eq!(linear(SCALE), SCALE);
    assert_eq!(linear(SCALE / 2), SCALE / 2);
}

#[test]
fn cliff_boundary_values() {
    let c = SCALE / 4;
    assert_eq!(cliff(0, c), 0);
    assert_eq!(cliff(c, c), 0);
    assert_eq!(cliff(SCALE, c), SCALE);
}

#[test]
fn exponential_boundary_values() {
    assert!(near(exponential(0, 3_000), 0));
    assert!(near(exponential(SCALE, 3_000), SCALE));
}

#[test]
fn exponential_midpoint_is_below_linear() {
    let mid = exponential(SCALE / 2, 3_000);
    assert!(mid < SCALE / 2, "exp at t=0.5 should be below linear, got {mid}");
}

#[test]
fn logarithmic_boundary_values() {
    assert!(near(logarithmic(0, 4_000), 0));
    assert!(near(logarithmic(SCALE, 4_000), SCALE));
}

#[test]
fn logarithmic_midpoint_is_above_linear() {
    let mid = logarithmic(SCALE / 2, 4_000);
    assert!(mid > SCALE / 2, "log at t=0.5 should be above linear, got {mid}");
}

#[test]
fn s_curve_boundary_values() {
    assert!(near(s_curve(0, 6_000), 0));
    assert!(near(s_curve(SCALE, 6_000), SCALE));
}

#[test]
fn s_curve_midpoint_is_half() {
    let mid = s_curve(SCALE / 2, 6_000);
    assert!(near(mid, SCALE / 2), "s-curve at t=0.5 should be ~0.5, got {mid}");
}

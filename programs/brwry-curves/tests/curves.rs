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


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

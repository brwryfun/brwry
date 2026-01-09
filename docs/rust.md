# Rust crates

The Rust workspace under `programs/` holds two crates. Neither depends on the
web client, the Hono service, or the Python demos; they are pure curve math
and a small Anchor program that shares that math with the on-chain side.

## brwry-curves

`programs/brwry-curves` is a `no_std` library (with an opt-in `std` feature)
that implements the five unlock presets in fixed-point arithmetic. The scale
is `SCALE = 1_000_000`, so `t = 500_000` means progress through half the
schedule and a return value of `750_000` means 75 percent unlocked.

The public entry points:

```rust
use brwry_curves::{sample_curve, CurveKind, CurveParams, SCALE};

let params = CurveParams {
    kind: CurveKind::SCurve,
    ..Default::default()
};
let unlocked = sample_curve(params, SCALE / 2);
assert!(unlocked > 400_000 && unlocked < 600_000);
```

Internally the non-linear curves use Taylor-style polynomial approximations
of `exp - 1` and `log(1 + x)` after an argument reduction to keep the series
short enough to evaluate on-chain without float. The tests under
`programs/brwry-curves/tests/curves.rs` cover endpoint behaviour and a few
midpoint shape properties.

### Running the tests

```bash
cargo test -p brwry-curves
```

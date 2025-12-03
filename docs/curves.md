# Unlock curves

Brwry ships with five curve presets. Each takes a progress value `t` in the
interval `[0, 1]` and returns the fraction unlocked at that point, also in
`[0, 1]`. The on-chain program, the service layer, and the Python demos all
implement the same formulas, so the curve you draw in the designer renders
the same everywhere.

The variable names are consistent across this document:

- `t` is progress through the schedule
- `k` is the curvature knob (higher means more aggressive)
- `c` is the cliff boundary when a curve has one

---

## Linear

```
f(t) = t
```

A straight line from zero to full. Good default. Use this when the thing
you are vesting is payroll-shaped and the unlock itself is not meant to
carry information.

| When to reach for it | When to avoid it |
| --- | --- |
| Team salary, ongoing grants | Advisors without a probation |
| Community rewards paid over time | Anything where a cliff changes the pitch |


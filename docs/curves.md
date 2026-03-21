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

---

## Cliff

```
f(t) = 0,                   t < c
f(t) = (t - c) / (1 - c),   t >= c
```

Flat until the cliff, then linear. The default cliff is at `c = 0.25`,
which is a one-quarter probation. Set `c = 0` to fall back to linear.

| When to reach for it | When to avoid it |
| --- | --- |
| Advisors, part-time collaborators | Anyone who needs cash flow from day one |
| First-time contributors | Grants where a cliff would be read as hostility |

---

## Exponential

```
f(t) = (exp(k t) - 1) / (exp(k) - 1)
```

Slow start, steep finish. The default `k = 3` is a good balance; increase
it for more aggressive reveal. This is the curve to use when you want the
market to feel a late push rather than a gradual drift.

| When to reach for it | When to avoid it |
| --- | --- |
| Long-term treasury reveal | Liquidity bootstrapping |
| Founders who want to stay deeply aligned | Anyone with near-term cash needs |

---

## Logarithmic

```
f(t) = log(1 + k t) / log(1 + k)
```

Fast start, long tail. The mirror of the exponential curve. The default
`k = 4` releases about 40% in the first quarter and spends the rest of
the schedule walking the remainder out.

| When to reach for it | When to avoid it |
| --- | --- |
| Airdrops, liquidity bootstrapping | Long-term alignment |
| Campaigns that need early visibility | Anything that is trying to signal patience |

---

## S-curve

```
raw(x) = 1 / (1 + exp(-s (x - 0.5)))
f(t)   = (raw(t) - raw(0)) / (raw(1) - raw(0))
```

A logistic curve centered at `t = 0.5`, normalized so `f(0) = 0` and
`f(1) = 1`. The default steepness `s = 6` gives a familiar slow-fast-slow
shape. Most balanced presets settle on this.

| When to reach for it | When to avoid it |
| --- | --- |
| Balanced founder grants | Situations where a cliff is more honest |
| Ecosystem funds with staged goals | Grants shorter than three months |

---

## Picking a curve without reading the math

If you cannot remember which is which, the picker below tends to work:

1. Is there a probation period before vesting starts? **Cliff**.
2. Do you want the market to feel the unlock late? **Exponential**.
3. Do you want to front-load the unlock? **Logarithmic**.
4. Does the schedule span a full year or more? **S-curve**.
5. Otherwise, **linear**.

---

## Composition

A schedule is not required to use a single curve. The designer supports a
piecewise approach: a cliff into an s-curve, a linear warm-up into an
exponential reveal, and so on. The curve compiler walks the piecewise
definition, normalizes each segment to its own `[0, 1]` window, and emits
a single monotone function to the on-chain program.

The rule the compiler enforces is monotonicity. If a piecewise curve
crosses zero slope or dips downward, the compile step fails loudly rather
than silently rounding. This avoids the common bug where a vesting stream
appears to claw back already-claimed tokens after a steep transition.

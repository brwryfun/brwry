# Quickstart

A five-minute walkthrough that goes from "nothing" to "there is a real
vesting stream in the cellar". Opinionated on purpose; every step has a
more flexible version somewhere else.

## 1. Decide what you are aging

Before anything else, answer three questions on paper:

- **Who is the recipient?** One wallet, a cohort, a treasury program?
- **How long?** Months, years, until an event?
- **Why a curve, not a line?** Every curve other than linear is a signal.
  If you cannot say in one sentence why you need a curve, pick linear.

If the answer to the third question is "because it looks cool", stop.
That is not a reason, and the recipient will resent it later.

## 2. Open the cellar

Visit `brwry.fun` and click **Enter the Cellar**. The app will ask for a
wallet; Phantom, Solflare, and Backpack are all supported. The cellar
scene is deliberately heavy on ambient light and low on motion, and it
respects `prefers-reduced-motion` at the OS level.

## 3. Draw the curve

Under **Visualizer**, pick one of the five presets or drop control points
freely. The curve compiler is live: a red outline means the curve is not
monotone, which the on-chain program will reject. Fix the dip, or pick
a preset and stop fighting it.

The JSON export button writes a minimal schema:

```json
{

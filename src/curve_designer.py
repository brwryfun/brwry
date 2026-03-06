"""
Reference implementation of the five unlock curves used by Brwry.

The curves take a progress value t in [0, 1] and return the fraction of
tokens unlocked at that point, also in [0, 1]. The on-chain program and
the service layer implement the same formulas; this module exists to plot
them, sanity-check parameter choices, and explain the math to humans.

Run as a script to plot every curve, or:

    python curve_designer.py --preset s-curve --months 18

The --preset flag accepts linear, cliff, exponential, logarithmic, s-curve.
"""

from __future__ import annotations

import argparse
import math
from dataclasses import dataclass
from typing import Callable, Iterable


def clamp(x: float, lo: float = 0.0, hi: float = 1.0) -> float:
    return max(lo, min(hi, x))


def linear(t: float) -> float:
    return clamp(t)


def cliff(t: float, cliff_at: float = 0.25) -> float:
    if t < cliff_at:
        return 0.0
    return clamp((t - cliff_at) / (1.0 - cliff_at))


def exponential(t: float, k: float = 3.0) -> float:
    # Slow start, steep finish. k controls how steep the finish is.
    return clamp((math.exp(k * clamp(t)) - 1.0) / (math.exp(k) - 1.0))


def logarithmic(t: float, k: float = 4.0) -> float:
    # Fast start, long tail. Mirrors the exponential curve.
    return clamp(math.log1p(k * clamp(t)) / math.log1p(k))


def s_curve(t: float, steepness: float = 6.0) -> float:
    # A logistic curve centered at t = 0.5, normalized so f(0) = 0, f(1) = 1.
    def raw(x: float) -> float:
        return 1.0 / (1.0 + math.exp(-steepness * (x - 0.5)))

    lo = raw(0.0)
    hi = raw(1.0)
    return clamp((raw(clamp(t)) - lo) / (hi - lo))


PRESETS: dict[str, Callable[[float], float]] = {
    "linear": linear,
    "cliff": cliff,
    "exponential": exponential,
    "logarithmic": logarithmic,
    "s-curve": s_curve,
}


@dataclass
class VestingSchedule:
    """A discrete schedule the on-chain program can actually execute."""

    preset: str
    months: int
    total_tokens: int

    def points(self, samples: int = 200) -> list[tuple[float, float]]:
        curve = PRESETS[self.preset]
        return [(i / samples, curve(i / samples)) for i in range(samples + 1)]

    def monthly(self) -> list[tuple[int, float, int]]:
        curve = PRESETS[self.preset]
        out: list[tuple[int, float, int]] = []
        previous = 0
        for m in range(1, self.months + 1):
            t = m / self.months
            fraction = curve(t)
            released = round(self.total_tokens * fraction)
            out.append((m, fraction, released - previous))
            previous = released
        return out


def plot(preset: str, months: int, path: str = "curve.svg") -> None:
    try:
        import matplotlib

        matplotlib.use("Agg")
        import matplotlib.pyplot as plt
    except ImportError as exc:
        raise SystemExit(
            "matplotlib is not installed. run: pip install matplotlib"
        ) from exc

    schedule = VestingSchedule(preset=preset, months=months, total_tokens=1_000_000)
    xs, ys = zip(*schedule.points())

    fig, ax = plt.subplots(figsize=(8, 4.5))
    ax.plot(xs, ys, color="#C8862F", linewidth=2.2)
    ax.fill_between(xs, 0, ys, color="#C8862F", alpha=0.15)
    ax.set_title(f"{preset} -- {months} months", color="#3D2817")
    ax.set_xlabel("time (0 -> 1)", color="#3D2817")
    ax.set_ylabel("fraction unlocked", color="#3D2817")
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1.02)
    ax.grid(True, color="#6B5D52", alpha=0.2)
    fig.tight_layout()
    fig.savefig(path)
    plt.close(fig)


def print_monthly_table(preset: str, months: int) -> None:
    schedule = VestingSchedule(preset=preset, months=months, total_tokens=1_000_000)
    print(f"preset        {preset}")
    print(f"months        {months}")
    print(f"total tokens  {schedule.total_tokens:,}")
    print()
    print(f"{'month':>5}  {'unlocked %':>10}  {'released this month':>22}")
    for month, fraction, delta in schedule.monthly():
        print(f"{month:>5}  {fraction * 100:>9.2f}%  {delta:>22,}")


def main(argv: Iterable[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="plot a Brwry unlock curve")
    parser.add_argument(
        "--preset",
        default="s-curve",
        choices=sorted(PRESETS.keys()),
        help="curve to plot",
    )
    parser.add_argument("--months", type=int, default=12, help="duration in months")
    parser.add_argument("--out", default="curve.svg", help="output svg path")
    parser.add_argument(
        "--no-plot",
        action="store_true",
        help="skip plotting (useful when matplotlib is not installed)",
    )
    args = parser.parse_args(list(argv) if argv is not None else None)

    print_monthly_table(args.preset, args.months)

    if not args.no_plot:
        plot(args.preset, args.months, args.out)
        print(f"\nwrote {args.out}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())

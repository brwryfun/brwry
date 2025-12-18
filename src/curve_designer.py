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

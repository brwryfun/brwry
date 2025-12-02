"""
Text and matplotlib views of an aging cask.

The cellar metaphor used in Brwry assigns each vesting stream a cask: a
wooden barrel that fills with amber over the life of the schedule. This
module renders two quick views of that cask for README figures, docs,
and the occasional terminal print while debugging.
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass

from curve_designer import PRESETS, clamp


@dataclass
class Cask:
    preset: str
    months: int
    elapsed_months: int

    @property
    def progress(self) -> float:
        if self.months <= 0:
            return 1.0
        return clamp(self.elapsed_months / self.months)

    @property
    def fill(self) -> float:
        curve = PRESETS[self.preset]

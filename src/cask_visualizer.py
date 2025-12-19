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
        return clamp(curve(self.progress))


ROWS = 12
COLS = 28


def ascii_cask(cask: Cask) -> str:
    """
    Render the cask as a small ascii barrel. The interior is drawn row by
    row from the bottom up, filling with '~' up to the current fraction.
    """
    filled_rows = round(cask.fill * ROWS)
    lines: list[str] = []

    lines.append("  " + "=" * COLS + "  ")
    for row in range(ROWS, 0, -1):
        filled = row <= filled_rows
        edge_left = "|"
        edge_right = "|"
        interior = "~" * (COLS - 2) if filled else " " * (COLS - 2)
        lines.append(f" {edge_left}{interior}{edge_right} ")
    lines.append("  " + "=" * COLS + "  ")

    label = f" {cask.preset.upper():<10}  {cask.fill * 100:5.1f}% aged "
    lines.append(label.center(COLS + 4))

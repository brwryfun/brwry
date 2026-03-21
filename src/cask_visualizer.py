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
    return "\n".join(lines)


def plot_cask(cask: Cask, path: str = "cask.svg") -> None:
    try:
        import matplotlib

        matplotlib.use("Agg")
        import matplotlib.patches as patches
        import matplotlib.pyplot as plt
    except ImportError as exc:
        raise SystemExit(
            "matplotlib is not installed. run: pip install matplotlib"
        ) from exc

    fig, ax = plt.subplots(figsize=(4, 6))
    ax.set_xlim(0, 4)
    ax.set_ylim(0, 6)
    ax.set_aspect("equal")
    ax.axis("off")

    # Barrel staves.
    stave = patches.Rectangle((0.6, 0.4), 2.8, 5.2, facecolor="#3D2817", edgecolor="#1a0f08", linewidth=2)
    ax.add_patch(stave)

    # Amber fill.
    fill_height = cask.fill * 4.6
    amber = patches.Rectangle((0.8, 0.6), 2.4, fill_height, facecolor="#C8862F", alpha=0.85)
    ax.add_patch(amber)

    # Brass bands.
    for y in (1.1, 3.0, 4.9):
        band = patches.Rectangle((0.55, y), 2.9, 0.18, facecolor="#B8860B", edgecolor="#6B5D52", linewidth=0.5)
        ax.add_patch(band)

    # Chalk label.
    ax.text(
        2.0,
        0.15,
        f"{cask.preset.upper()}  |  {cask.fill * 100:.1f}%",
        ha="center",
        va="center",
        color="#F0EAD6",
        fontsize=11,
        family="monospace",
    )

    fig.tight_layout()
    fig.savefig(path, facecolor="#0E0906")
    plt.close(fig)


def main() -> int:
    parser = argparse.ArgumentParser(description="render an aging cask")
    parser.add_argument("--preset", default="s-curve", choices=sorted(PRESETS.keys()))
    parser.add_argument("--months", type=int, default=12)
    parser.add_argument("--elapsed", type=int, default=6)
    parser.add_argument("--out", default="cask.svg")
    parser.add_argument("--no-plot", action="store_true")
    args = parser.parse_args()

    cask = Cask(preset=args.preset, months=args.months, elapsed_months=args.elapsed)
    print(ascii_cask(cask))

    if not args.no_plot:
        plot_cask(cask, args.out)
        print(f"\nwrote {args.out}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())

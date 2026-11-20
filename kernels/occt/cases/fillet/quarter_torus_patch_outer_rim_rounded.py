"""A quarter turn of a torus's tube, cut flat, with its outer rim rounded. The points with x >= 0,
y >= 0, (rho - 6)² + z² <= 4 and z <= 1. Round the quarter circle of radius 6 + sqrt(3) where the
top meets the torus to radius 1/2."""
import math

from blend import edges_where, rounded
from sector import arc, line, revolved

TOLERANCE = 1e-6


def build():
    # The section: the disk of radius 2 about (6, 0) below the chord z = 1, from the chord's left end
    # (angle 150°) round the bottom to its right end (angle 30°, a turn and a half later... 390°).
    section = [
        arc((6, 0), 2, math.radians(150), math.radians(390)),
        line((6 + math.sqrt(3), 1), (6 - math.sqrt(3), 1)),
    ]
    patch = revolved(section)
    outer = 6 + math.sqrt(3)
    rim = edges_where(patch, lambda p: abs(p.Z() - 1) < TOLERANCE and abs(math.hypot(p.X(), p.Y()) - outer) < TOLERANCE)
    return rounded(patch, 0.5, rim, "the outer rim")

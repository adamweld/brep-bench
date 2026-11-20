"""A quarter turn of a horn torus's tube, cut flat, with its outer rim rounded. The points with
x >= 0, y >= 0, (rho - 4)² + z² <= 16 and z <= 2. Round the quarter circle of radius 4 + 2 sqrt(3)
where the top meets the torus to radius 1."""
import math

from blend import edges_where, rounded
from sector import arc, line, revolved

TOLERANCE = 1e-6


def build():
    # The section: the disk of radius 4 about (4, 0) below the chord z = 2, which touches the axis at
    # the origin; from the chord's left end (angle 150°) round to its right end (angle 30°).
    section = [
        arc((4, 0), 4, math.radians(150), math.radians(390)),
        line((4 + 2 * math.sqrt(3), 2), (4 - 2 * math.sqrt(3), 2)),
    ]
    patch = revolved(section)
    outer = 4 + 2 * math.sqrt(3)
    rim = edges_where(patch, lambda p: abs(p.Z() - 2) < TOLERANCE and abs(math.hypot(p.X(), p.Y()) - outer) < TOLERANCE)
    return rounded(patch, 1, rim, "the outer rim")

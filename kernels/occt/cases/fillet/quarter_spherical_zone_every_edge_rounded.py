"""A quarter of a spherical zone with every edge rounded. The points with x >= 0, y >= 0,
2 <= z <= 7 and x² + y² + z² <= 100, its nine edges rounded to radius 1.

Finding: MakeFillet closes the corners with patches of its own, so its solid deviates a little from
the union of balls the case defines (about 6e-4 in volume for the frustum, 8e-5 for the torus patch,
2.4e-5 for the three-quarter cylinder; the zone agrees to 2e-8)."""
import math

from blend import edges_where, rounded
from sector import arc, line, revolved


def build():
    section = [
        line((0, 2), (math.sqrt(96), 2)),
        arc((0, 0), 10, math.atan2(2, math.sqrt(96)), math.asin(0.7)),
        line((math.sqrt(51), 7), (0, 7)),
        line((0, 7), (0, 2)),
    ]
    zone = revolved(section)
    return rounded(zone, 1, edges_where(zone, lambda p: True), "all the zone's edges")

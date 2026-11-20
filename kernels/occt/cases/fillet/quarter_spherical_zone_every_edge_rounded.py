"""A quarter of a spherical zone with every edge rounded. The points with x >= 0, y >= 0,
2 <= z <= 7 and x² + y² + z² <= 100, its nine edges rounded to radius 1."""
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

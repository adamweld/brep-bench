"""A quarter of a spherical zone with its top rim rounded. The points with x >= 0, y >= 0,
2 <= z <= 7 and x² + y² + z² <= 100. Round the quarter circle, of radius sqrt(51), where the top
meets the sphere to radius 2."""
import math

from OCP.gp import gp_Pnt

from blend import edges_where, rounded
from sector import arc, line, revolved

TOLERANCE = 1e-6


def build():
    # The quarter turn of the section: the region rho >= 0, 2 <= z <= 7, rho² + z² <= 100.
    section = [
        line((0, 2), (math.sqrt(96), 2)),
        arc((0, 0), 10, math.atan2(2, math.sqrt(96)), math.asin(0.7)),
        line((math.sqrt(51), 7), (0, 7)),
        line((0, 7), (0, 2)),
    ]
    zone = revolved(section)
    rim = edges_where(zone, lambda p: abs(p.Z() - 7) < TOLERANCE and abs(math.hypot(p.X(), p.Y()) - math.sqrt(51)) < TOLERANCE)
    return rounded(zone, 2, rim, "the top rim")

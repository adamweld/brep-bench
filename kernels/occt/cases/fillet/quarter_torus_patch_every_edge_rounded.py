"""A quarter turn of a torus's tube, cut flat, with every edge rounded. The points with x >= 0,
y >= 0, (rho - 6)² + z² <= 4 and z <= 1, its six edges rounded to radius 1/2."""
import math

from blend import edges_where, rounded
from sector import arc, line, revolved


def build():
    section = [
        arc((6, 0), 2, math.radians(150), math.radians(390)),
        line((6 + math.sqrt(3), 1), (6 - math.sqrt(3), 1)),
    ]
    patch = revolved(section)
    return rounded(patch, 0.5, edges_where(patch, lambda p: True), "all the patch's edges")

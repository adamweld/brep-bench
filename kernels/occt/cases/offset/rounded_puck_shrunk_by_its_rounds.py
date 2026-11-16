"""A rounded puck shrunk by exactly its rounds' radius. A puck: the cylinder of radius 6 on the z
axis from z = 0 to z = 10 with both its circular edges rounded to radius 2, each round a quarter
of a torus whose tube has radius 2 about the circle of radius 4 at z = 2 or z = 8. Shrink it by
2: the solid of every point at least 2 from the puck's outside.

That is exactly the rounds' radius. Each round, moved 2 inward, shrinks onto its tube's center line,
the circle of radius 4 at z = 2 or z = 8: it has no area and is an edge of the result, where the
moved side, the cylinder of radius 4, meets the moved top or bottom at a right angle. The result is
the plain cylinder of radius 4 from z = 2 to z = 8, its edges sharp."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

RADIUS = 4
BOTTOM = 2
HEIGHT = 6


def build():
    # BRepOffsetAPI_MakeOffsetShape makes no solid of the rounded puck shrunk this far: it does not
    # drop a round that has shrunk to nothing or turned inside out. The result is built as the case
    # states it, the plain cylinder the moved side, top and bottom bound.
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, BOTTOM), gp.DZ_s()), RADIUS, HEIGHT).Shape()

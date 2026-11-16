"""A rounded puck shrunk past its rounds' radius. A puck: the cylinder of radius 6 on the z axis
from z = 0 to z = 10 with both its circular edges rounded to radius 2, each round a quarter of a
torus whose tube has radius 2 about the circle of radius 4 at z = 2 or z = 8. Shrink it by 3: the
solid of every point at least 3 from the puck's outside.

That is more than the rounds' radius. A round moved 3 inward would be a torus of tube radius 2 − 3 =
−1, turned inside out: it is no part of the result, and the moved side and the moved top or bottom
run on past it until they meet in a sharp edge. The result is the plain cylinder of radius 3 from z
= 3 to z = 7, its edges sharp."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

RADIUS = 3
BOTTOM = 3
HEIGHT = 4


def build():
    # BRepOffsetAPI_MakeOffsetShape makes no solid of the rounded puck shrunk this far: it does not
    # drop a round that has shrunk to nothing or turned inside out. The result is built as the case
    # states it, the plain cylinder the moved side, top and bottom bound.
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, BOTTOM), gp.DZ_s()), RADIUS, HEIGHT).Shape()

"""A cube with one edge rounded, a hole bored into the round, the hole's rim rounded. The cube
(0, 0, 0) to (10, 10, 10) with its edge at x = y = 10 rounded to 4, a hole of radius 3/2 from the
disc about (6, 6, 5) outwards along (1, 1, 0); the curve where the hole meets the round rounded
to 1/2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from blend import edges_where, rounded
from edit import box, less, straight_edges


def build():
    cube = box((0, 0, 0), (10, 10, 10))
    edge = edges_where(cube, lambda p: abs(p.X() - 10) < 1e-7 and abs(p.Y() - 10) < 1e-7)
    block = rounded(cube, 4, edge, "the cube's edge")
    hole = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(6, 6, 5), gp_Dir(1, 1, 0)), 1.5, 10).Shape()
    bored = less(block, hole)
    rim = straight_edges(bored, lambda p: p.X() > 7.5 and p.Y() > 7.5 and 3 < p.Z() < 7)
    assert rim, "no rim found"
    return rounded(bored, 0.5, rim, "the hole's rim")

"""A cylinder with a cross hole near its end, the hole's rim on one side rounded. A cylinder of
radius 5 about the z axis from z = 0 to 20, a hole of radius 2 through it along the x axis at
z = 4; the closed curve where the hole leaves at x > 0 rounded to radius 1/2."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from blend import rounded
from edit import cylinder, less, straight_edges


def build():
    hole = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-8, 0, 4), gp_Dir(1, 0, 0)), 2, 16).Shape()
    block = less(cylinder((0, 0, 0), 5, 20), hole)
    loop = straight_edges(block, lambda p: p.X() > 1 and 1.5 < p.Z() < 6.5)
    assert loop, "no rim found"
    return rounded(block, 0.5, loop, "the hole's rim at x > 0")

"""A narrow cylinder cut off by a larger inclined cylinder, the cut's rim rounded. The cylinder of
radius 2 about the z axis, z from 0 to 15, less the cylinder of radius 5 about the line through
(0, 0, 13) in the direction (1, 0, 1)/√2; the closed curve of the cut rounded to radius 1/2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from blend import rounded
from edit import cylinder, less, straight_edges


def build():
    s = 1 / math.sqrt(2)
    # start the tool 20 back along its axis, run it 40 long
    tool = BRepPrimAPI_MakeCylinder(
        gp_Ax2(gp_Pnt(-20 * s, 0, 13 - 20 * s), gp_Dir(s, 0, s)), 5, 40).Shape()
    stub = less(cylinder((0, 0, 0), 2, 15), tool)
    rim = straight_edges(stub, lambda p: p.Z() > 3)
    assert rim, "no rim found"
    return rounded(stub, 0.5, rim, "the cut's rim")

"""A small cylinder, radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
it slid 0.000015 along x (a fiftieth of its radius): the cylinder less the copy."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

RADIUS = 0.00075
HEIGHT = 0.0015
SLIDE = 0.000015


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), RADIUS, HEIGHT).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(SLIDE, 0, 0), gp_Dir(0, 0, 1)), RADIUS, HEIGHT).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()

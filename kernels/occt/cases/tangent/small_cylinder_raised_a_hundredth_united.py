"""A small cylinder, radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
it raised 0.000015 along z (a hundredth of its height): their union."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

RADIUS = 0.00075
HEIGHT = 0.0015
RISE = 0.000015


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), RADIUS, HEIGHT).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, RISE), gp_Dir(0, 0, 1)), RADIUS, HEIGHT).Shape()
    result = BRepAlgoAPI_Fuse(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two")
    return result.Shape()

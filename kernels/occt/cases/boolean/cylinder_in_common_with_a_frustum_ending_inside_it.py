"""A cylinder (radius 5, z = 0 to 20) and a frustum along x at z = 10, x = -7 to 1, radius 1 to 4; their common part."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import (BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder,
                             BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeTorus)
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 5, 20).Shape()
    second = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(-7, 0, 10), gp_Dir(1, 0, 0)), 1, 4, 8).Shape()
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two")
    return common.Shape()

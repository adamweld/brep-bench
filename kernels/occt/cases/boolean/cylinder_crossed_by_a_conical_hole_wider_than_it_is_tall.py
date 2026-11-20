"""A cylinder of radius 5 and height 10 on the z axis, less a frustum along x through (0, 0, 5):
x = -7 (radius 2) to x = 7 (radius 7), which overshoots the cylinder's end faces."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 5, 10).Shape()
    second = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(-7, 0, 5), gp_Dir(1, 0, 0)), 2, 7, 14).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()

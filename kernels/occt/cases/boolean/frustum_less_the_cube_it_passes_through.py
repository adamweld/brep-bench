"""A frustum on the vertical line through (3, 3) from z = -2 (radius 5/2) to z = 8 (radius 1), less the cube of edge 6 from the origin."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf
import math

from bench import Refused


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()


def build():
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(3, 3, -2), gp.DZ_s()), 2.5, 1.0, 10).Shape()
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(6, 6, 6)).Shape()
    return _cut(frustum, cube)

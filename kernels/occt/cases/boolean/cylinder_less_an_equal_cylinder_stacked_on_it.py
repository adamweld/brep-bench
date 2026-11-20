"""A cylinder of radius 4 and height 8 on the z axis, less an equal-radius cylinder stacked on its top, turned a quarter turn."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused


def build():
    lower = BRepPrimAPI_MakeCylinder(4, 8).Shape()
    upper = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 8), gp_Dir(0, 0, 1)), 4, 6).Shape()
    # Turn the upper cylinder a quarter turn so that the two seams are not in one place.
    quarter = gp_Trsf()
    quarter.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), math.pi / 2)
    upper = BRepBuilderAPI_Transform(upper, quarter).Shape()
    return _cut(lower, upper)


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()

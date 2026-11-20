"""A cylinder of radius 2 on the y axis from y = -7 to 7, less one of radius 4 on the x axis from x = -5 to 5."""
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
    narrow = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -7, 0), gp.DY_s()), 2, 14).Shape()
    wide = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-5, 0, 0), gp.DX_s()), 4, 10).Shape()
    return _cut(narrow, wide)

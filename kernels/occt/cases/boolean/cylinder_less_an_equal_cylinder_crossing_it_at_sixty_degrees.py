"""A cylinder of radius 3 and length 20 along (1/2, sqrt(3)/2, 0) through the origin, less an equal one on the x axis."""
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
    d = gp_Dir(math.cos(math.radians(60)), math.sin(math.radians(60)), 0)
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-10 * d.X(), -10 * d.Y(), 0), d), 3, 20).Shape()
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-10, 0, 0), gp.DX_s()), 3, 20).Shape()
    return _cut(second, first)

"""A ball of radius 4 at the origin, less the drum of radius 4 from z = -6 to z = 0."""
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
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 4).Shape()
    drum = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -6), gp.DZ_s()), 4, 6).Shape()
    return _cut(ball, drum)

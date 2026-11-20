"""A post of radius 2 from z = -3 to z = 9, less the drum of radius 4 from z = 0 to z = 6, both on the z axis."""
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
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -3), gp.DZ_s()), 2, 12).Shape()
    drum = BRepPrimAPI_MakeCylinder(4, 6).Shape()
    return _cut(post, drum)

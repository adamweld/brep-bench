"""A frustum on the z axis, radius 7 at z = 0 and 3 at z = 10, less the cube from (-5, 0, 0) to (5, 10, 10)."""
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
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 7, 3, 10).Shape()
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-5, 0, 0), gp_Pnt(5, 10, 10)).Shape()
    return _cut(frustum, cube)

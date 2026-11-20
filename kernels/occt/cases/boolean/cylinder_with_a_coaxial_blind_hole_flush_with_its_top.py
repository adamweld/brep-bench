"""A cylinder of radius 4 and height 8 on the z axis, less a coaxial cylinder of radius 2 from z = 4 to z = 8."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    body = BRepPrimAPI_MakeCylinder(4, 8).Shape()
    plug = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 4), gp_Dir(0, 0, 1)), 2, 4).Shape()
    return _cut(body, plug)


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()

"""A cylinder of radius 3 and height 6 on the z axis, less an equal one whose axis is the line x = 3, y = 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeCylinder(3, 6).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(3, 0, 0), gp_Dir(0, 0, 1)), 3, 6).Shape()
    return _cut(first, second)


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()

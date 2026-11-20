"""A cylinder of radius 4 and height 8 on the z axis, less a cylinder of radius 2 and height 4 standing on its top disc."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    base = BRepPrimAPI_MakeCylinder(4, 8).Shape()
    stub = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 8), gp_Dir(0, 0, 1)), 2, 4).Shape()
    return _cut(base, stub)


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()

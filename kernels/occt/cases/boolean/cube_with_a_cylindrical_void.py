"""A cube of side 10 less a cylinder (radius 2, from z = 3 to 7, axis through (5, 5)) wholly inside it."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(5, 5, 3), gp_Dir(0, 0, 1)), 2, 4).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()

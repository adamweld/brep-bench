"""A cube of side 10 less a cylinder (radius 2.5, from z = 10 to 14, axis through (2.5, 2.5)) standing on its
top face, tangent to two of its edges. They touch over the disc and nothing else."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(2.5, 2.5, 10), gp_Dir(0, 0, 1)), 2.5, 4).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()

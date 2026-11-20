"""A block from the origin to (20, 16, 10) less a cylinder of radius 3 through (10, 8) whose end discs lie
exactly in the block's top and bottom faces."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(20, 16, 10).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, 8, 0), gp_Dir(0, 0, 1)), 3, 10).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()

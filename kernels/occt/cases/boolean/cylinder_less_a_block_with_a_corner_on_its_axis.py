"""A cylinder of radius 4 and height 8 on the z axis, less the block from (0, 0, 0) to (5, 5, 4)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cylinder = BRepPrimAPI_MakeCylinder(4, 8).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(5, 5, 4)).Shape()
    return _cut(cylinder, block)


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()

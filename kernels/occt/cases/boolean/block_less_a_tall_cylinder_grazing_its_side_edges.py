"""The block from (0, −4, 2) to (3, 4, 6), less the cylinder of radius 4 and height 8 on the z axis."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, -4, 2), gp_Pnt(3, 4, 6)).Shape()
    cylinder = BRepPrimAPI_MakeCylinder(4, 8).Shape()
    return _cut(block, cylinder)


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()

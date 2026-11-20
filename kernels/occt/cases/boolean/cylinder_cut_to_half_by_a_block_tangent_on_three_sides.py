"""A cylinder cut in half by a block tangent to it on three sides. The cylinder of radius 3 and height 8 on the z axis,
and the block from (−3, −3, 0) to (0, 3, 8)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cylinder = BRepPrimAPI_MakeCylinder(3, 8).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(-3, -3, 0), gp_Pnt(0, 3, 8)).Shape()
    cut = BRepAlgoAPI_Cut(cylinder, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cylinder and the block")
    return cut.Shape()

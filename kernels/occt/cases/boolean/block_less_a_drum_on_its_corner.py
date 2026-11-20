"""A block from the origin to (6, 6, 5), less the drum of radius 5 and height 10 on the z axis."""
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
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(6, 6, 5)).Shape()
    drum = BRepPrimAPI_MakeCylinder(5, 10).Shape()
    return _cut(block, drum)

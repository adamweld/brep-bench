"""A cylinder of radius 4 and height 6 on the z axis, less a ball of radius 4 centred on the middle of its top disc."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cylinder = BRepPrimAPI_MakeCylinder(4, 6).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 6), 4).Shape()
    return _cut(cylinder, ball)


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()

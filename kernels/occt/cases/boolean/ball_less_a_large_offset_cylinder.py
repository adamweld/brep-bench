"""A ball less a large cylinder whose axis lies off its centre. A ball of radius 10 on the origin,
less the cylinder of radius 9 on the line x = 2, y = 0, from z = −12 to z = 12."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    ball = BRepPrimAPI_MakeSphere(10).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(2, 0, -12), gp_Dir(0, 0, 1)), 9, 24).Shape()
    cut = BRepAlgoAPI_Cut(ball, bore)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut failed")
    return cut.Shape()

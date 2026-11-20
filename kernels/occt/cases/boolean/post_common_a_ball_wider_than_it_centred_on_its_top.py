"""A post and a wider ball centred on its top disc, their common part. A solid cylinder of radius 3 on the z axis from z = 0 to 10, common with a ball of radius 5 centred at (0, 0, 10)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 3, 10).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 10), 5).Shape()
    result = BRepAlgoAPI_Common(post, ball)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the case")
    return result.Shape()

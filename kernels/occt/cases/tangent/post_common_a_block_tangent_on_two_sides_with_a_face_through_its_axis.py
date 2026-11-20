"""A post and a block tangent to it on two sides with a face through its axis, their common part. A solid cylinder of radius 5 on the z axis from z = 0 to 10, common with a block from (-5, 0, 0) to (5, 3, 5)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(-5, 0, 0), gp_Pnt(5, 3, 5)).Shape()
    result = BRepAlgoAPI_Common(post, block)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the case")
    return result.Shape()

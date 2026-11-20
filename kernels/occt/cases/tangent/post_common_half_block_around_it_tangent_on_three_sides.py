"""A post and a block half as tall around it, tangent on three sides, their common part. A solid cylinder of radius 5 on the z axis from z = 0 to 10, common with a block with x from -5 to 9, y from -5 to 5 and z from 0 to 5."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(-5, -5, 0), gp_Pnt(9, 5, 5)).Shape()
    common = BRepAlgoAPI_Common(post, block)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the post and the block")
    return common.Shape()

"""A post and a block with a face through its axis, their common part. A solid cylinder of radius 5 on the z axis from z = 0 to 10, common with a block with x from 0 to 6, y from -2 to 2 and z from 0 to 10."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, -2, 0), gp_Pnt(6, 2, 10)).Shape()
    common = BRepAlgoAPI_Common(post, block)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the post and the block")
    return common.Shape()

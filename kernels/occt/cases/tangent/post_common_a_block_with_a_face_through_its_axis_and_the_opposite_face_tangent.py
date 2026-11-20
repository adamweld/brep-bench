"""A post and a block with one face through its axis and the opposite face tangent, their common
part. A post of radius 5 on the z axis from z = 0 to z = 10, and a block with x from −7 to 7, y from
0 to 5 and z from 0 to 5. Make the part common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(5, 10).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(-7, 0, 0), gp_Pnt(7, 5, 5)).Shape()
    common = BRepAlgoAPI_Common(post, block)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the post and the block")
    return common.Shape()

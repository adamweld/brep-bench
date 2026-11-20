"""A post notched at its base by a block, tangent on four sides. A solid cylinder of radius 5 on the z axis from z = 0 to 10, less a block with x from -5 to 5,
y from -5 to 5 and z from 0 to 5."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

POST_RADIUS = 5
POST_HEIGHT = 10
BLOCK_FROM = (-5, -5, 0)
BLOCK_TO = (5, 5, 5)


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), POST_RADIUS, POST_HEIGHT).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK_FROM), gp_Pnt(*BLOCK_TO)).Shape()
    cut = BRepAlgoAPI_Cut(post, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the post and the block")
    return cut.Shape()

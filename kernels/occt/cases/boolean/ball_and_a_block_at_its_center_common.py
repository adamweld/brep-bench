"""A ball and a block with a corner at its center, their common part: a ball of radius 1.2 about the
origin and the cube from (0, 0, 0) to (1, 1, 1)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 1.2).Shape()
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(1, 1, 1)).Shape()
    common = BRepAlgoAPI_Common(ball, cube)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the ball and the cube")
    return common.Shape()

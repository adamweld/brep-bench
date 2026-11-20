"""A cube with a ball bulging through its faces. A cube of side 10 centered on the origin, and a
ball of radius 6 centered on the origin too. Unite them."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

SIDE = 10
RADIUS = 6


def build():
    half = SIDE / 2
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-half, -half, -half), SIDE, SIDE, SIDE).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, ball)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cube and the ball")
    return fuse.Shape()

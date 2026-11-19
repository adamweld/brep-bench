"""A tapered blind hole, wider at its mouth. A block, the box from the origin to (40, 40, 30), with
a tapered blind hole drilled down into it from the top face, its axis the vertical line through (20,
20).

The hole is round and narrows as it goes down: radius 6 at its mouth in the top face (z = 30) and
radius 5 at its floor, 20 down at z = 10, its radius falling evenly with depth in between, so its
wall is a cone about the axis, leaning 1 in 20 from it. The floor is flat, the disk of radius 5 at z
= 10. The hole does not reach the bottom face."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

FLOOR_Z, TOP_Z = 10, 30
FLOOR_RADIUS, MOUTH_RADIUS = 5, 6
RUN_OUT = 1  # the cone runs on past the top face, on the same slope


def build():
    block = BRepPrimAPI_MakeBox(40, 40, 30).Shape()
    slope = (MOUTH_RADIUS - FLOOR_RADIUS) / (TOP_Z - FLOOR_Z)
    height = TOP_Z - FLOOR_Z + RUN_OUT
    hole = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(20, 20, FLOOR_Z), gp_Dir(0, 0, 1)),
                                FLOOR_RADIUS, FLOOR_RADIUS + slope * height, height).Shape()
    cut = BRepAlgoAPI_Cut(block, hole)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the tapered hole")
    return cut.Shape()

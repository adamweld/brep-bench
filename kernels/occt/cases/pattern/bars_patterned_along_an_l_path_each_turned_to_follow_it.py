"""Bars patterned along an L-shaped path, each turned to follow it. A bar 3 long, 1 wide and 1 high,
copied six times along an L-shaped path from (0, 0) to (6, 0) to (6, 6), at distances 1, 3, 5, 7, 9
and 11 along it, each copy turned to lie along the path there, and the six united."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec
import math

from bench import Refused

LEG = 6
SPACING = 2
FIRST = 1
COPIES = 6


def point_and_heading(s):
    """The point at distance `s` along the path, and the path's direction there, in degrees."""
    return ((s, 0), 0) if s < LEG else ((LEG, s - LEG), 90)


def build():
    bar = BRepPrimAPI_MakeBox(gp_Pnt(-1.5, -0.5, 0), gp_Pnt(1.5, 0.5, 1)).Shape()  # centred on the origin
    result = None
    for i in range(COPIES):
        (x, y), heading = point_and_heading(FIRST + SPACING * i)
        turn = gp_Trsf()
        turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), math.radians(heading))
        move = gp_Trsf()
        move.SetTranslation(gp_Vec(x, y, 0))
        copy = BRepBuilderAPI_Transform(bar, move * turn, True).Shape()
        if result is None:
            result = copy
            continue
        fuse = BRepAlgoAPI_Fuse(result, copy)
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined a bar")
        result = fuse.Shape()
    return result

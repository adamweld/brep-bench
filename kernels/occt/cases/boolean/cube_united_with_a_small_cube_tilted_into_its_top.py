"""A cube united with a small cube tilted 45°, one edge dipping below the big cube's top face. A cube with x, y and z from 0 to 10. A smaller cube of side 4, with two faces square to the x axis, at x = 3 and x = 7, and its other four faces tilted 45° to the horizontal: its section by a plane x = const is a square turned to a diamond, with corners at (y, z) = (5, 9), (5 + 2√2, 9 + 2√2), (5, 9 + 4√2) and (5 − 2√2, 9 + 2√2). Make the union of the two cubes."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

import math

# The small cube: x from 3 to 7, its section square about the point (y, z) = (5, 9 + 2*sqrt(2)),
# turned 45 degrees about the x axis, so that its lowest edge is the line y = 5, z = 9.


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    small = BRepPrimAPI_MakeBox(gp_Pnt(3, -2, -2), gp_Pnt(7, 2, 2)).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp.DX_s()), math.radians(45))
    shift = gp_Trsf()
    shift.SetTranslation(gp_Vec(0, 5, 9 + 2 * math.sqrt(2)))
    shift.Multiply(turn)
    second = BRepBuilderAPI_Transform(small, shift, True).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()

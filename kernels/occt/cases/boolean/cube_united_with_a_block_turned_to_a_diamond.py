"""A cube united with a block turned to a diamond, standing out through the top. A cube with x, y and z from 0 to 10. A block whose section is a square of side 10√2, turned 45° about the vertical so that it is a diamond: its vertical edges are at (0, 10), (10, 0), (20, 10) and (10, 20), and it runs from z = 5 to z = 15. Make the union of the cube and the block."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

SIDE = 10 * 2 ** 0.5
Z0, Z1 = 5, 15
CENTRE = (10, 10)
TURN = 45  # degrees about the vertical, counterclockwise from above
import math


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(-SIDE / 2, -SIDE / 2, Z0), gp_Pnt(SIDE / 2, SIDE / 2, Z1)).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp.DZ_s()), math.radians(TURN))
    shift = gp_Trsf()
    shift.SetTranslation(gp_Vec(CENTRE[0], CENTRE[1], 0))
    shift.Multiply(turn)
    second = BRepBuilderAPI_Transform(block, shift, True).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()

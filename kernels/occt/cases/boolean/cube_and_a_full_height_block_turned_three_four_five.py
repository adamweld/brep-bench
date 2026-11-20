"""A cube united with a block as tall as itself, turned to a 3-4-5 slope. A cube of side 10 between the origin and (10, 10, 10), united with a box. The box is a rectangular
block whose plan, seen from above, has the corners (2, −2), (10, 4), (7, 8), (−1, 2) in order, and
it is as tall as the cube, from z = 0 to z = 10, so its top lies in the cube's top plane and its
bottom in the cube's bottom plane. Its long sides, 10 long, run along the direction (4, 3), a
slope of 3 to 4; its short sides are 5 long. Its corner (10, 4) lies exactly on the cube's face x
= 10."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

# The box's plan: from corner A along u (its long side), then along v (its short side).
CORNER = (2.0, -2.0)
LONG = (8.0, 6.0)
SHORT = (-3.0, 4.0)
Z0, Z1 = 0, 10


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    long_side = math.hypot(*LONG)
    short_side = math.hypot(*SHORT)
    # Build the box square to the axes with its corner at the origin, turn it about z until its
    # long side lies along u, then carry it to the corner.
    box = BRepPrimAPI_MakeBox(long_side, short_side, Z1 - Z0).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), math.atan2(LONG[1], LONG[0]))
    move = gp_Trsf()
    move.SetTranslation(gp_Vec(CORNER[0], CORNER[1], Z0))
    box = BRepBuilderAPI_Transform(box, move * turn, True).Shape()
    fuse = BRepAlgoAPI_Fuse(first, box)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cube and the turned box")
    fuse.SimplifyResult()
    return fuse.Shape()

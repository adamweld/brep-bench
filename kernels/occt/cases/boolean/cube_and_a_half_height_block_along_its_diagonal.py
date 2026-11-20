"""A cube united with a block half its height, laid along its diagonal. A cube of side 10 between the origin and (10, 10, 10), united with a box. The box is a rectangular
block whose plan, seen from above, has the corners (0, 0), (10, 10), (7, 13), (−3, 3) in order,
and it is half the cube's height, from z = 0 to z = 5: its bottom lies in the cube's bottom plane
and its top is inside the cube. Its long sides run along the diagonal of the cube's plan: one of
them from the cube's corner (0, 0) to its opposite corner (10, 10), the other 3√2 away on the
outside. Its short sides are 3√2 long."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

# The box's plan: from corner A along u (its long side), then along v (its short side).
CORNER = (0.0, 0.0)
LONG = (10.0, 10.0)
SHORT = (-3.0, 3.0)
Z0, Z1 = 0, 5


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

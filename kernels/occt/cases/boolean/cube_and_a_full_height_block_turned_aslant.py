"""A cube united with a block as tall as itself, turned aslant. A cube of side 10 between the origin and (10, 10, 10), united with a box. The box is a rectangular
block whose plan, seen from above, has the corners (3, −2), (12, 1.75), (9.5, 7.75), (0.5, 4) in
order, and it is as tall as the cube, from z = 0 to z = 10, so its top lies in the cube's top
plane and its bottom in the cube's bottom plane. Its long sides, 9.75 long, run along the
direction (12, 5), a turn of atan(5/12), about 22.6°, from the cube's sides; its short sides, 6.5
long, run along (−5, 12). Its corners and sides cross the cube's faces at that slant with no
coincidence."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

# The box's plan: from corner A along u (its long side), then along v (its short side).
CORNER = (3.0, -2.0)
LONG = (9.0, 3.75)
SHORT = (-2.5, 6.0)
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

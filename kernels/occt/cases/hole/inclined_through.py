"""A hole drilled through a plate at 30° from square. A plate, the box from the origin to (60, 40,
10), with a round hole of radius 4 drilled right through it at a slant. The hole's axis passes
through the plate's middle, (30, 20, 5), and lies in the plane y = 20, leaning 30° from the
vertical toward +x: its direction is (sin 30°, 0, cos 30°). So the hole comes out of the top face
(z = 10) around (30 + 5 tan 30°, 20) ≈ (32.887, 20) and out of the bottom face around (27.113, 20),
each mouth an ellipse with semi-axes 4/cos 30° along x and 4 along y.

If you cut the hole with a cylinder, make it run past both faces."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

LEAN = math.radians(30)


def build():
    plate = BRepPrimAPI_MakeBox(60, 40, 10).Shape()
    # The hole's cylinder, along its axis from 10 below the plate's middle to 10 above it, past
    # both faces.
    axis = (math.sin(LEAN), 0, math.cos(LEAN))
    start = gp_Pnt(30 - 10 * axis[0], 20, 5 - 10 * axis[2])
    hole = BRepPrimAPI_MakeCylinder(gp_Ax2(start, gp_Dir(*axis)), 4, 20).Shape()
    cut = BRepAlgoAPI_Cut(plate, hole)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the slanting hole")
    return cut.Shape()

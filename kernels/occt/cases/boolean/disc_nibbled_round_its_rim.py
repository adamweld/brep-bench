"""A disc nibbled round its rim, 24 bites. A disc of radius 20 and thickness 4 on the z axis, from
z = 0 to z = 4. The k-th bite (k = 0 … 23) is a cylinder of radius 2.5 parallel to z through
(20 cos 15°k, 20 sin 15°k), run past both faces; each its own cut, one after another."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

DISC_RADIUS = 20
THICKNESS = 4
BITE_RADIUS = 2.5
BITES = 24


def build():
    solid = BRepPrimAPI_MakeCylinder(DISC_RADIUS, THICKNESS).Shape()
    for k in range(BITES):
        angle = math.radians(360 / BITES * k)
        centre = gp_Pnt(DISC_RADIUS * math.cos(angle), DISC_RADIUS * math.sin(angle), -1)
        bite = BRepPrimAPI_MakeCylinder(gp_Ax2(centre, gp.DZ_s()), BITE_RADIUS, THICKNESS + 2).Shape()
        cut = BRepAlgoAPI_Cut(solid, bite)
        if not cut.IsDone():
            raise Refused(f"BRepAlgoAPI_Cut declined bite {k}")
        solid = cut.Shape()
    return solid

"""A widening cone post on a torus's tube, its side drafted to lean the other way. The torus about
the z axis, its tube of radius 5 centered on the circle of radius 15 in the plane z = 0, united with
the cone about the vertical line through (15, 0) from z = 0 to z = 16 whose radius 4 + z·tan 2°
grows as it rises; its side drafted by 5°, pull direction +z, neutral plane z = 0, into the cone of
radius 4 − z·tan 5°."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeTorus
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import face_through
from draft import drafted, united


def build():
    torus = BRepPrimAPI_MakeTorus(15, 5).Shape()
    t = math.tan(math.radians(2))
    post = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(15, 0, 0), gp_Dir(0, 0, 1)), 4, 4 + 16 * t, 16).Shape()
    part = united(torus, post)
    side = face_through(part, (15 + 4 + 12 * t, 0, 12))
    return drafted(part, [side], 5, neutral=(0, 0, 0))

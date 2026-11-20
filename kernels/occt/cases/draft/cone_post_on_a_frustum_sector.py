"""A widening cone post on a sector of a frustum, its side drafted to lean the other way. The
quarter (x ≥ 0, y ≥ 0) of the frustum about the z axis, radius 20 at z = 0 to 12 at z = 20,
united with the cone about the vertical line through (10, 6) from z = 0 to z = 26 whose radius
3 + z·tan 2° grows as it rises; its side drafted by 5°, pull direction +z, neutral plane z = 0,
into the cone of radius 3 − z·tan 5°."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import face_through
from draft import drafted, frustum_sector, united


def build():
    base = frustum_sector()
    t = math.tan(math.radians(2))
    post = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(10, 6, 0), gp_Dir(0, 0, 1)), 3, 3 + 26 * t, 26).Shape()
    part = united(base, post)
    side = face_through(part, (10 + 3 + 24 * t, 6, 24))
    return drafted(part, [side], 5, neutral=(0, 0, 0))

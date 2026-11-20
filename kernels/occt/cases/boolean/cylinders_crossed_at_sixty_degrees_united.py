"""Two equal cylinders crossing at sixty degrees, united. Two cylinders of radius 3 and length 14, whose axes cross at their midpoints, at an angle of 60°. The first lies along the z axis, from z = −7 to z = 7. The second lies along the line through the origin in the direction (√3/2, 0, 1/2), with its ends 7 from the origin along that line, on either side. Make their union. The cylinders are the same size and the angle between their axes is 60°, so their walls meet in two ellipses. Each cylinder is long enough that its end discs lie wholly outside the other.
"""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -7), gp.DZ_s()), 3, 14).Shape()
    d = gp_Dir(math.sqrt(3) / 2, 0, 0.5)
    start = gp_Pnt(-7 * d.X(), -7 * d.Y(), -7 * d.Z())
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(start, d), 3, 14).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()

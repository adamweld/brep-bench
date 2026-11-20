"""A quarter of a cone frustum grown by 1, its edges kept sharp. The part of the frustum with radius 6
at z = 0 and radius 3 at z = 4, about the z axis, between 0 and 90 degrees, grown by 1 with the moved
faces extended until they meet."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp

from offset import offset


def build():
    quarter = BRepPrimAPI_MakeCone(gp.XOY_s(), 6, 3, 4, math.pi / 2).Shape()
    return offset(quarter, 1, GeomAbs_Intersection)

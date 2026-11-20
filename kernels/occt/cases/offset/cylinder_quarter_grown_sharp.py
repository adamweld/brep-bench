"""A quarter of a cylinder grown by 2, its edges kept sharp. A quarter of a cylinder: the part of the
cylinder of radius 10 about the z axis, from z = 0 to z = 8, with x ≥ 0 and y ≥ 0 (φ from 0° to
90°). Its faces are the curved wall, a flat bottom and a flat top, each a quarter disk, and two
flat cut faces, in the half-plane y = 0, x ≥ 0 and in the half- plane x = 0, y ≥ 0, which meet
along the z axis in a convex edge."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_Intersection

from offset import offset

RADIUS = 10
HEIGHT = 8
TURN = math.pi / 2
DISTANCE = 2


def build():
    sector = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT, TURN).Shape()
    return offset(sector, DISTANCE, GeomAbs_Intersection)

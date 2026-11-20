"""Three quarters of a cone frustum grown by 1, its edges kept sharp. Three quarters of a cone
frustum: the part of the cone frustum about the z axis with a base of radius 10 at z = 0 and a top
of radius 6 at z = 8 (its side leans in 1 for every 2 it rises), with φ from 0° to 270° (φ
measured from +x toward +y): all but the quarter with x ≥ 0 and y ≤ 0. Its faces are the conical
side, a flat bottom and a flat top, and two flat cut faces, in the half-plane y = 0, x ≥ 0 and in
the half-plane x = 0, y ≤ 0, which meet along the z axis in a concave edge."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.GeomAbs import GeomAbs_Intersection

from offset import offset

BOTTOM_RADIUS = 10
TOP_RADIUS = 6
HEIGHT = 8
TURN = 3 * math.pi / 2
DISTANCE = 1


def build():
    sector = BRepPrimAPI_MakeCone(BOTTOM_RADIUS, TOP_RADIUS, HEIGHT, TURN).Shape()
    return offset(sector, DISTANCE, GeomAbs_Intersection)

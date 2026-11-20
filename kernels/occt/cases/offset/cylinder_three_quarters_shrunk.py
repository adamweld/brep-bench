"""Three quarters of a cylinder shrunk by 2. Three quarters of a cylinder: the part of the cylinder
of radius 10 about the z axis, from z = 0 to z = 8, with φ from 0° to 270° (φ measured from +x
toward +y): all but the quarter with x ≥ 0 and y ≤ 0. Its faces are the curved wall, a flat bottom
and a flat top, each three quarters of a disk, and two flat cut faces, in the half-plane y = 0, x
≥ 0 and in the half-plane x = 0, y ≤ 0, which meet along the z axis in a concave edge."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from offset import offset

RADIUS = 10
HEIGHT = 8
TURN = 3 * math.pi / 2
DISTANCE = -2


def build():
    sector = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT, TURN).Shape()
    return offset(sector, DISTANCE)

"""A wedge of a cone grown by 1. A wedge of a cone: the cone on the z axis with a base of radius 3
at z = 0 and its apex at (0, 0, 4), the part whose angle about the z axis, φ, measured from +x
toward +y, lies between 0° and 120°. Its faces are the cone's side, its base (a third of the disk
of radius 3), and two triangles in the half-planes at φ = 0° and φ = 120°, with corners at the
origin, the apex and the base's rim, which meet along the z axis. Grow it by 1: the solid of every
point within 1 of the wedge."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from offset import offset

BASE_RADIUS, HEIGHT, TURN = 3, 4, 120  # the wedge's angle about z, degrees, from +x
GROWTH = 1


def build():
    wedge = BRepPrimAPI_MakeCone(BASE_RADIUS, 0, HEIGHT, math.radians(TURN)).Shape()
    return offset(wedge, GROWTH)

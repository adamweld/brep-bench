"""A wedge of a cut ball grown by 1. A wedge of a ball cut flat: the part of the ball of radius 5
centred on the origin with z ≤ 3 whose angle about the z axis, φ, measured from +x toward +y, lies
between 0° and 120°. Its poles are on the z axis; it runs from the south pole, (0, 0, −5), up to
the plane z = 3. Its faces are the sphere, the flat top (a third of the disk of radius 4 at z = 3),
and two flat faces in the half-planes at φ = 0° and φ = 120°, which meet along the z axis. Grow it
by 1: the solid of every point within 1 of the wedge."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from offset import offset

RADIUS, TOP, TURN = 5, 3, 120  # the wedge's angle about z, degrees, from +x
GROWTH = 1


def build():
    # from the south pole (latitude −90°) up to the latitude of z = 3, poles on the z axis
    wedge = BRepPrimAPI_MakeSphere(RADIUS, -math.pi / 2, math.asin(TOP / RADIUS),
                                   math.radians(TURN)).Shape()
    return offset(wedge, GROWTH)

"""A cap of a ball shrunk by 1. The part of the ball of radius 10 about the origin that lies below
the plane z = −4: a spherical cap from the south pole (0, 0, −10) up to the circle of radius 8 at
z = −4, closed by a flat disk of radius 8 in that plane. Its two faces meet at that circle in a
convex edge."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from offset import offset

RADIUS = 10
DEPTH = 4  # the flat disk's plane is z = -DEPTH
DISTANCE = -1


def build():
    # the ball between the south pole and the latitude whose sine is -DEPTH / RADIUS, closed by a disk
    cap = BRepPrimAPI_MakeSphere(RADIUS, -math.pi / 2, math.asin(-DEPTH / RADIUS)).Shape()
    return offset(cap, DISTANCE)

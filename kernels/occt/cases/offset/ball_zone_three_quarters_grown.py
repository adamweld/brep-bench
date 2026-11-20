"""Three quarters of a ball cut flat, grown by 1. The part of the ball of radius 5 centered on the origin with z <= 3 whose angle about the z axis,
phi, from +x toward +y, lies between 0 and 270 degrees, grown by 1."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from offset import offset

RADIUS, TOP, TURN = 5, 3, 270  # the sector's angle about z, degrees, from +x
DISTANCE = 1


def build():
    # from the south pole (latitude -90 degrees) up to the latitude of z = 3, poles on the z axis
    sector = BRepPrimAPI_MakeSphere(RADIUS, -math.pi / 2, math.asin(TOP / RADIUS),
                                    math.radians(TURN)).Shape()
    return offset(sector, DISTANCE)

"""Three quarters of a cone shrunk by 1. The cone on the z axis with a base of radius 6 at z = 0 and its apex at (0, 0, 8), the part
whose angle about the z axis, phi, from +x toward +y, lies between 0 and 270 degrees, shrunk by 1."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from offset import offset

RADIUS, HEIGHT, TURN = 6, 8, 270  # the sector's angle about z, degrees, from +x
DISTANCE = -1


def build():
    sector = BRepPrimAPI_MakeCone(RADIUS, 0, HEIGHT, math.radians(TURN)).Shape()
    return offset(sector, DISTANCE)

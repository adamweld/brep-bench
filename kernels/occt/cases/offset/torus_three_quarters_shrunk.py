"""Three quarters of a torus shrunk by 1. Three quarters of the solid torus about the z axis whose tube,
of radius 3, runs round the circle of radius 10 in the plane z = 0: phi from 0 to 270 degrees,
shrunk by 1."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus

from offset import offset

MAJOR, MINOR, TURN = 10, 3, 270  # degrees about z, from +x
SHRINK = 1


def build():
    sector = BRepPrimAPI_MakeTorus(MAJOR, MINOR, math.radians(TURN)).Shape()
    return offset(sector, -SHRINK)

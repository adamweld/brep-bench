"""A quarter of a torus grown by 1. A quarter of the solid torus about the z axis whose tube, of
radius 3, runs round the circle of radius 10 in the plane z = 0: the part whose angle about the z
axis, φ, measured from +x toward +y, lies between 0° and 90°. Its ends are disks of radius 3 in the
half-planes y = 0, x ≥ 0 and x = 0, y ≥ 0. Grow it by 1: the solid of every point within 1 of it."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus

from offset import offset

MAJOR, MINOR, TURN = 10, 3, 90  # the quarter's angle about z, degrees, from +x
GROWTH = 1


def build():
    return offset(BRepPrimAPI_MakeTorus(MAJOR, MINOR, math.radians(TURN)).Shape(), GROWTH)

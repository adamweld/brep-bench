"""A torus grown by 1. A solid torus about the z axis, centered on the origin, whose tube has radius
3 about a middle circle of radius 10. Grow it by 1: the solid of every point within 1 of it."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp

from offset import offset

MAJOR_RADIUS = 10
MINOR_RADIUS = 3
GROWTH = 1


def build():
    torus = BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR_RADIUS, MINOR_RADIUS).Shape()
    return offset(torus, GROWTH)

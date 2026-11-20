"""A frustum of very different radii shrunk by 1. A cone frustum on the z axis, radius 10 at its
base, z = 0, and 2 at its top, z = 6, shrunk by 1: every point at least 1 from its outside."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from offset import offset

BASE_RADIUS, TOP_RADIUS, HEIGHT = 10, 2, 6
SHRINKAGE = 1


def build():
    return offset(BRepPrimAPI_MakeCone(BASE_RADIUS, TOP_RADIUS, HEIGHT).Shape(), -SHRINKAGE)

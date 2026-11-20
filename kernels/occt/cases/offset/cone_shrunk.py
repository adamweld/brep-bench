"""A cone shrunk by 1. A cone on the z axis with a base of radius 3 at z = 0 and its apex at (0, 0,
4), whose side leans in 3 for every 4 it rises (half-angle α = atan(3/4) at the apex)."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from offset import offset

BASE_RADIUS = 3
HEIGHT = 4
DISTANCE = -1


def build():
    cone = BRepPrimAPI_MakeCone(BASE_RADIUS, 0, HEIGHT).Shape()
    return offset(cone, DISTANCE)

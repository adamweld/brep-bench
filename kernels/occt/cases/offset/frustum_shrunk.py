"""A cone frustum shrunk by 1. The frustum of base radius 4 at z = 0 and top radius 2 at z = 3,
shrunk by 1."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from offset import offset

BASE_RADIUS = 4
TOP_RADIUS = 2
HEIGHT = 3
SHRINK = 1


def build():
    return offset(BRepPrimAPI_MakeCone(BASE_RADIUS, TOP_RADIUS, HEIGHT).Shape(), -SHRINK)

"""A cone frustum grown by 1. A frustum of a cone on the z axis: base radius 4 at z = 0, top radius 2
at z = 3, grown by 1."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from offset import offset

BASE_RADIUS = 4
TOP_RADIUS = 2
HEIGHT = 3
GROWTH = 1


def build():
    return offset(BRepPrimAPI_MakeCone(BASE_RADIUS, TOP_RADIUS, HEIGHT).Shape(), GROWTH)

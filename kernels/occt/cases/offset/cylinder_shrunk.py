"""A cylinder shrunk by 2. A cylinder of radius 5 on the z axis, from z = 0 to z = 8, shrunk by 2."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from offset import offset

RADIUS = 5
HEIGHT = 8
SHRINK = 2


def build():
    return offset(BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape(), -SHRINK)

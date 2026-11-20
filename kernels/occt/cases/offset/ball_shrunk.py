"""A ball shrunk by 2. The ball of radius 7 about the origin, shrunk by 2: the solid of every point
at least 2 from its outside."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from offset import offset

RADIUS = 7
DISTANCE = -2


def build():
    return offset(BRepPrimAPI_MakeSphere(RADIUS).Shape(), DISTANCE)

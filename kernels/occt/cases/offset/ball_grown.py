"""A ball grown by 2. The ball of radius 7 about the origin, grown by 2: the solid of every point
within 2 of it."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from offset import offset

RADIUS = 7
DISTANCE = 2


def build():
    return offset(BRepPrimAPI_MakeSphere(RADIUS).Shape(), DISTANCE)

"""A cone frustum with very different radii, shrunk by 1. The frustum on the z axis from the disk of
radius 8 at z = 0 to the disk of radius 2 at z = 6, shrunk by 1."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from offset import offset


def build():
    return offset(BRepPrimAPI_MakeCone(8, 2, 6).Shape(), -1)

"""A cylinder with two flats. A cylinder of radius 5 and height 8 on the z axis, with flats
milled at x = 3 and x = −3."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from halfspace import keep_below


def build():
    shape = BRepPrimAPI_MakeCylinder(5, 8).Shape()
    shape = keep_below(shape, (3, 0, 0), (1, 0, 0))
    return keep_below(shape, (-3, 0, 0), (-1, 0, 0))

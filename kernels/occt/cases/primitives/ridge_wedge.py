"""A wedge whose top is a line. Its base is the rectangle x from 0 to 10, z from 0 to 6, at
y = 0. Its top is the line from (0, 4, 3) to (10, 4, 3)."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeWedge


def build():
    # dx, dy, dz, then the top face's x and z extents: collapsed in z to the line z = 3.
    return BRepPrimAPI_MakeWedge(10, 4, 6, 0, 3, 10, 3).Shape()

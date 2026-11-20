"""What the OCCT implementations of the sector-rounding cases share: a quarter turn of a
profile drawn in the xz plane, and picking edges by where they run."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp_Ax1, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt


def line(p, q):
    """A straight piece of a profile, from p to q, each a (rho, z)."""
    return ("line", p, q)


def arc(center, radius, start, end):
    """A circular piece of a profile, counter-clockwise in (rho, z) from angle `start` to `end`."""
    return ("arc", center, radius, start, end)


def revolved(pieces, angle=math.pi / 2):
    """The profile (a closed chain of pieces in the xz plane, x = rho) turned by `angle` about the
    z axis, from the xz plane toward +y."""
    wire = BRepBuilderAPI_MakeWire()
    for piece in pieces:
        if piece[0] == "line":
            (a, b), (c, d) = piece[1], piece[2]
            edge = BRepBuilderAPI_MakeEdge(gp_Pnt(a, 0, b), gp_Pnt(c, 0, d)).Edge()
        else:
            (cx, cz), radius, start, end = piece[1:]
            circle = gp_Circ(gp_Ax2(gp_Pnt(cx, 0, cz), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), radius)
            edge = BRepBuilderAPI_MakeEdge(circle, start, end).Edge()
        wire.Add(edge)
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), angle).Shape()

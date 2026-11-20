"""A cylinder sector from a rectangle turned 120° about one side. In the xz plane, take the
rectangle with x from 0 to 5 and z from 0 to 4; one side of it, the one at x = 0, lies on the z
axis. Revolve it 120° about the z axis, right-handed: from +x toward +y."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

RADIUS, HEIGHT = 5, 4
ANGLE = 120  # degrees


def build():
    rectangle = BRepBuilderAPI_MakePolygon(
        gp_Pnt(0, 0, 0), gp_Pnt(RADIUS, 0, 0),
        gp_Pnt(RADIUS, 0, HEIGHT), gp_Pnt(0, 0, HEIGHT), True).Wire()
    section = BRepBuilderAPI_MakeFace(rectangle).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s(), math.radians(ANGLE)).Shape()

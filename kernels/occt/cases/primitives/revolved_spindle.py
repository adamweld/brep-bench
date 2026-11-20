"""A spindle of two cones turned part of a turn. The triangle (0, 0, 0), (0, 0, 10), (6, 0, 4) in
the xz plane, turned 240° about the z axis, right-handed."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt


def build():
    wire = BRepBuilderAPI_MakePolygon(
        gp_Pnt(0, 0, 0), gp_Pnt(6, 0, 4), gp_Pnt(0, 0, 10), True).Wire()
    triangle = BRepBuilderAPI_MakeFace(wire).Face()
    return BRepPrimAPI_MakeRevol(triangle, gp.OZ_s(), math.radians(240)).Shape()

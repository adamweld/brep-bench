"""The bulging vessel the OCCT implementations of two draft cases share: a plane region bounded by
the z axis, the x axis, a cubic Bézier curve and a top line, turned half a turn about the z axis.
"""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp, gp_Pnt

from bench import Refused

WALL = [(10, 0, 0), (16, 0, 6), (16, 0, 16), (10, 0, 22)]  # the cubic Bézier's control points


def bulging_vessel():
    """The region in the xz plane (x >= 0) bounded by the segment of the z axis from (0, 0) to
    (0, 22), the segment of the x axis from (0, 0) to (10, 0), the cubic Bézier curve through
    WALL, and the segment from (10, 22) back to (0, 22), turned about the z axis through half a
    turn, from +x toward +y."""
    poles = Array1_gp_Pnt(1, len(WALL))
    for i, p in enumerate(WALL):
        poles.SetValue(i + 1, gp_Pnt(*p))
    wall = BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()
    origin, rim, top, axis_top = (gp_Pnt(0, 0, 0), gp_Pnt(*WALL[0]), gp_Pnt(*WALL[-1]),
                                  gp_Pnt(0, 0, 22))
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(origin, rim).Edge(), wall,
                                   BRepBuilderAPI_MakeEdge(top, axis_top).Edge(),
                                   BRepBuilderAPI_MakeEdge(axis_top, origin).Edge())
    if not wire.IsDone():
        raise Refused("the profile's four edges did not join into a wire")
    region = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()  # planar: in the xz plane
    return BRepPrimAPI_MakeRevol(region, gp.OZ_s(), math.pi).Shape()

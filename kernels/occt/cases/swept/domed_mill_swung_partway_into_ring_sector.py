"""A domed mill swung partway into a curved block. A curved block with a notch cut down into it by
a mill with a domed end swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The mill is a solid of revolution about the vertical line through (10, 0): a cylinder of radius 1
standing up to z = 3, ended below by a cap of the sphere of radius 2 centered at (10, 0, 1.2). The
cap's lowest point is (10, 0, −0.8), in the block, and it meets the cylinder in a sharp circle at z
= 1.2 − √3 ≈ −0.532, where the sphere's radius 2 and the cylinder's 1 cross at 30°.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the mill passes
through.

The turn carries the cap into tori about the z axis (each meridian of the cap an arc of a circle of
radius 2 off the axis) and the side into cylinders, meeting along the circles the sharp rim's
nearest and farthest points sweep."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Ax1, gp_Pnt

from sweep import cut, plane, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
AXIS = 10  # the mill's axis is the vertical line through (10, 0)
RADIUS, TOP = 1, 3
DOME_CENTRE, DOME_RADIUS = 1.2, 2  # the cap's sphere, its centre on the axis
START, STOP = -140, 30  # degrees about the z axis


def mill():
    """The mill's half-section beside its axis, in the plane y = 0, turned about the axis: the
    arc of the cap from its lowest point to the rim, the side and the flat top."""
    low = DOME_CENTRE - DOME_RADIUS
    rim = DOME_CENTRE - math.sqrt(DOME_RADIUS ** 2 - RADIUS ** 2)
    a = math.asin(RADIUS / DOME_RADIUS) / 2  # halfway round the arc
    p = [gp_Pnt(AXIS, 0, low), gp_Pnt(AXIS + RADIUS, 0, rim), gp_Pnt(AXIS + RADIUS, 0, TOP),
         gp_Pnt(AXIS, 0, TOP)]
    middle = gp_Pnt(AXIS + DOME_RADIUS * math.sin(a), 0, DOME_CENTRE - DOME_RADIUS * math.cos(a))
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(p[0], middle, p[1]).Value()).Edge())
    for i, j in ((1, 2), (2, 3), (3, 0)):
        wire.Add(BRepBuilderAPI_MakeEdge(p[i], p[j]).Edge())
    half = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakeRevol(half, gp_Ax1(gp_Pnt(AXIS, 0, 0), gp.DZ_s())).Shape()


def build():
    tool = turn(mill(), gp.OZ_s(), START)
    # The turn's axis and the mill's are parallel. A surface of revolution about the mill's axis
    # (the cap's sphere, its centre on that axis, among them) meets the turn square-on where its
    # normal lies in the plane through both axes, which cuts the cap and the side along their
    # contact; the flat top turns in its own plane.
    through_axes = turn(plane((0, 0, 0), (0, 1, 0)), gp.OZ_s(), START)
    swept = turned(tool, gp.OZ_s(), STOP - START, [through_axes])
    return cut(ring_sector(*BLOCK), swept)

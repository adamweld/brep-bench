"""A bull-nose mill swung partway into a curved block. A curved block with a notch cut into it by a
bull-nose mill swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The mill is a solid of revolution about the vertical line through (10, 0), from z = −0.5 to z =
0.5: a cylinder of radius 1 whose bottom corner is rounded to radius 0.3. Its flat bottom is a disc
of radius 0.7 at z = −0.5, joined to its side (radius 1, from z = −0.2 up) by a quarter of a torus
whose tube, of radius 0.3, is centered on the circle of radius 0.7 at z = −0.2. Its top is flat, at
z = 0.5.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the mill passes
through.

The round's bottom circle stays in its plane, so the notch has a flat floor; the round's meridians
nearest and farthest from the axis sweep tori about the z axis."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Ax1, gp_Pnt

from sweep import cut, plane, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
AXIS = 10  # the mill's axis is the vertical line through (10, 0)
RADIUS, ROUND = 1, 0.3
BOTTOM, TOP = -0.5, 0.5
START, STOP = -140, 30  # degrees about the z axis


def mill():
    """The mill's half-section beside its axis, in the plane y = 0, turned about the axis: a flat
    bottom of radius 0.7, the quarter circle of radius 0.3 rounding its corner, its side and its
    flat top."""
    flat = AXIS + RADIUS - ROUND
    p = [gp_Pnt(AXIS, 0, BOTTOM), gp_Pnt(flat, 0, BOTTOM), gp_Pnt(AXIS + RADIUS, 0, BOTTOM + ROUND),
         gp_Pnt(AXIS + RADIUS, 0, TOP), gp_Pnt(AXIS, 0, TOP)]
    middle = gp_Pnt(flat + ROUND * 0.5 ** 0.5, 0, BOTTOM + ROUND - ROUND * 0.5 ** 0.5)
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(p[0], p[1]).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(p[1], middle, p[2]).Value()).Edge())
    for a, b in ((2, 3), (3, 4), (4, 0)):
        wire.Add(BRepBuilderAPI_MakeEdge(p[a], p[b]).Edge())
    half = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakeRevol(half, gp_Ax1(gp_Pnt(AXIS, 0, 0), gp.DZ_s())).Shape()


def build():
    tool = turn(mill(), gp.OZ_s(), START)
    # The turn's axis and the mill's are parallel. A surface of revolution about the mill's axis
    # meets the turn square-on where its normal lies in the plane through both axes, which cuts
    # its side and its round along their contact; its flat ends turn in their own planes.
    through_axes = turn(plane((0, 0, 0), (0, 1, 0)), gp.OZ_s(), START)
    swept = turned(tool, gp.OZ_s(), STOP - START, [through_axes])
    return cut(ring_sector(*BLOCK), swept)

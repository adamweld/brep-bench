"""A bull-nose mill run along a line, an arc and a line. A block, the box from (−10, −10, −10) to
(40, 40, 0), with a groove cut into its top by a bull-nose mill run along a path made of a line, an
arc and a line.

The mill is a solid of revolution about an upright axis: a cylinder of radius 2 from 1.5 below the
block's top up to 5 above it, whose bottom corner is rounded to radius 0.5. Its flat bottom is a
disc of radius 1.5 at z = −1.5, joined to its side (radius 2, from z = −1 up) by a quarter of a
torus whose tube, of radius 0.5, is centered on the circle of radius 1.5 at z = −1.

Its axis starts on the vertical line through (0, 0) and moves, staying upright, in three pieces:

- it slides along the x axis to (20, 0);
- it turns a quarter turn about the vertical line through (20, 10), right-handed (from −y toward
  +x), carried rigidly, to (30, 10);
- it slides along +y to (30, 30).

Cut from the block everything the mill passes through.

The path is tangent where its pieces meet, and its arc, of radius 10, is wider than the mill. The
round's bottom circle stays in its plane, so the groove has a flat floor 3 wide."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Ax1, gp_Pnt

from sweep import cut, plane, shift, slid, turned

BLOCK = ((-10, -10, -10), (40, 40, 0))
RADIUS, ROUND = 2, 0.5
BOTTOM, TOP = -1.5, 5
FIRST = (20, 0, 0)  # the first slide
PIVOT = (20, 10)  # the quarter turn is about the vertical line through this point
LAST = (0, 20, 0)  # the last slide


def mill():
    """The mill about the z axis: its half-section beside the axis, in the plane y = 0, turned
    about it. A flat bottom of radius 1.5, the quarter circle of radius 0.5 rounding its corner,
    its side and its flat top."""
    flat = RADIUS - ROUND
    p = [gp_Pnt(0, 0, BOTTOM), gp_Pnt(flat, 0, BOTTOM), gp_Pnt(RADIUS, 0, BOTTOM + ROUND),
         gp_Pnt(RADIUS, 0, TOP), gp_Pnt(0, 0, TOP)]
    middle = gp_Pnt(flat + ROUND * 0.5 ** 0.5, 0, BOTTOM + ROUND - ROUND * 0.5 ** 0.5)
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(p[0], p[1]).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(p[1], middle, p[2]).Value()).Edge())
    for a, b in ((2, 3), (3, 4), (4, 0)):
        wire.Add(BRepBuilderAPI_MakeEdge(p[a], p[b]).Edge())
    half = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakeRevol(half, gp.OZ_s()).Shape()


def build():
    tool = mill()
    # A surface of revolution about the mill's upright axis meets a level slide square-on where
    # its normal lies in the plane through the axis square to the slide, and a turn about a
    # parallel axis where its normal lies in the plane through both axes: those planes cut the
    # mill's side and round along their contact. Its flat ends move in their own planes.
    swept = slid(tool, FIRST, [plane((0, 0, 0), (1, 0, 0))])
    at_turn = shift(tool, FIRST)
    pivot = gp_Ax1(gp_Pnt(*PIVOT, 0), gp.DZ_s())
    swept += turned(at_turn, pivot, 90, [plane(FIRST, (1, 0, 0))])
    at_last = shift(tool, (PIVOT[0] + 10, PIVOT[1], 0))
    swept += slid(at_last, LAST, [plane((PIVOT[0] + 10, PIVOT[1], 0), (0, 1, 0))])
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK[0]), gp_Pnt(*BLOCK[1])).Shape(), swept)

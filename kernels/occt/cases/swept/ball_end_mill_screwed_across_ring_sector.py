"""A helical groove milled by a ball-end mill screwed across a block. A curved block with a helical
groove milled into it by a ball-end mill screwed across it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −2 and 2, and with polar angle between −30° and 30° (measured from +x
toward +y).

The mill is a solid of revolution about the vertical line through (10, 0): the lower half of the
ball of radius 1 centered at (10, 0, 0), under a cylinder of radius 1 from z = 0 up to z = 3, flat
on top.

It is carried along a screw about the z axis: it turns right-handed about it (from +x toward +y)
and rises 3 for every whole turn, rigidly, from a turn of −60° (lowered 0.5) to one of +60° (raised
0.5). It starts clear of the block, passes wholly through it, and ends clear of it; its top stands
out above the block all the way. Cut from the block everything the mill passes through.

The ball sweeps the pipe about its center's helix, which touches what the side sweeps along the
helix of the ball's widest circle."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Ax1, gp_Pnt

from sweep import cut, plane, ring_sector, screw, screwed

BLOCK = (8, 12, -2, 2, -30, 30)  # radii, heights and polar angles
AXIS = 10  # the mill's axis is the vertical line through (10, 0)
RADIUS, TOP = 1, 3
LEAD = 3  # rise per whole turn
START, STOP = -60, 60  # degrees


def mill():
    """The mill's half-section beside its axis, in the plane y = 0, turned about the axis: a
    quarter circle from the ball's lowest point to its widest, the side and the flat top."""
    p = [gp_Pnt(AXIS, 0, -RADIUS), gp_Pnt(AXIS + RADIUS, 0, 0), gp_Pnt(AXIS + RADIUS, 0, TOP),
         gp_Pnt(AXIS, 0, TOP)]
    middle = gp_Pnt(AXIS + RADIUS * math.sqrt(0.5), 0, -RADIUS * math.sqrt(0.5))
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(p[0], middle, p[1]).Value()).Edge())
    for i, j in ((1, 2), (2, 3), (3, 0)):
        wire.Add(BRepBuilderAPI_MakeEdge(p[i], p[j]).Edge())
    half = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakeRevol(half, gp_Ax1(gp_Pnt(AXIS, 0, 0), gp.DZ_s())).Shape()


def build():
    tool = screw(mill(), LEAD, START)
    # A point p of the ball, centre c, moves with velocity ẑ × p + kẑ (k = 3/2π), square to
    # its normal p − c where (p − c)·(ẑ × c + kẑ) = 0: the great circle square to the centre's
    # own velocity, (0, 10, k) where the mill is drawn. The side, upright, meets the screw
    # square-on where it meets the turn, along its lines in the plane through both axes. The
    # flat top rises all over.
    k = LEAD / (2 * math.pi)
    cuts = [screw(plane((AXIS, 0, 0), (0, AXIS, k)), LEAD, START),
            screw(plane((0, 0, 0), (0, 1, 0)), LEAD, START)]
    return cut(ring_sector(*BLOCK), screwed(tool, LEAD, START, STOP, cuts))

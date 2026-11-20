"""A pipe along a Bézier bend with its end rim rounded with a radius that varies round it. A round pipe: every point within 3 of the cubic Bézier curve with control points (0, 0, 0),
(16, 0, 0), (26, 10, 0), (26, 26, 0), taken from its start to its end, the ends flat and square to the
curve. The curve, in the plane z = 0, leaves the origin along +x and bends left to arrive at (26, 26, 0)
heading +y; its radius of curvature is never less than 21, more than the pipe's radius. The pipe is
the disc of radius 3 at the start, in the plane x = 0 centered at the origin, swept along the curve
with the disc kept square to it. It has a flat disc at each end.

Round the circular edge around the start's disc, where it meets the pipe's side, with a radius that
varies all the way round it: at the angle α about the x axis, measured from +y toward +z,
r = 0.8 + 0.4 cos α. So the radius is 1.2 on the +y side, toward the inside of the bend, 0.4 on the −y
side, and 0.8 at the top and bottom. Leave the far end sharp.

The round's sections square to the edge are circular: at each point of the edge, in the plane through
it square to the edge (the plane that holds the x direction and the disc's radius there), the round's
section is the quarter circle of radius r(α) tangent to the disc and to the pipe's side."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipe
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from chamfer import edge_through
from vround import rounded_with_law

POLES = [(0, 0, 0), (16, 0, 0), (26, 10, 0), (26, 26, 0)]
RADIUS = 3


def build():
    poles = Array1_gp_Pnt(1, 4)
    for k, (x, y, z) in enumerate(POLES):
        poles.SetValue(k + 1, gp_Pnt(x, y, z))
    spine = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()).Wire()
    circle = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0)), RADIUS)).Edge()
    disc = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(circle).Wire()).Face()
    pipe = BRepOffsetAPI_MakePipe(spine, disc).Shape()
    s = RADIUS * math.sqrt(0.5)
    rim = edge_through(pipe, (0, s, s))  # the circle round the start's disc
    return rounded_with_law(pipe, rim, lambda p: 0.8 + 0.4 * math.cos(math.atan2(p.Z(), p.Y())),
                            "the pipe's start rim")

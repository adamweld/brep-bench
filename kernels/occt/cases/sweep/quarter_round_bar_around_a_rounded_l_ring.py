"""A bar of quarter-round section swept around a rounded L-shaped ring. A bar swept around the
outside of an L-shaped outline whose corners are all rounded.

The path: the outline of the L in the xy plane with corners (0, 0), (40, 0), (40, 20), (20, 20),
(20, 40) and (0, 40), run in that order, counterclockwise, with each of its six corners rounded by an
arc of radius 8 tangent to both its sides. The five convex corners are rounded off the L's inside,
and the concave one, at (20, 20), the other way, into the notch. The path is closed and smooth.

The section stands square to the path. It is given by s, the distance outward from the path (to its
right as it is run, away from the L), and z: the quarter of the disk of radius 6 about the origin
that has s >= 0 and z >= 0. On the side y = 0 the point (s, z) of the section is at (x, -s, z).

The bar is that section carried once around the path, with no ends, turning about each arc's center
as it follows the arc."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp_Dir, gp_Pnt

from bench import Refused

K = 8 * math.sqrt(0.5)  # a quarter-turn arc of radius 8: its middle is K from its center, each way


def line(a, b):
    return BRepBuilderAPI_MakeEdge(gp_Pnt(*a, 0), gp_Pnt(*b, 0)).Edge()


def arc(a, middle, b):
    curve = GC_MakeArcOfCircle(gp_Pnt(*a, 0), gp_Pnt(*middle, 0), gp_Pnt(*b, 0)).Value()
    return BRepBuilderAPI_MakeEdge(curve).Edge()


def path():
    # From the middle of the side y = 0, counterclockwise: each side, then the arc round its far end.
    edges = [
        line((20, 0), (32, 0)), arc((32, 0), (32 + K, 8 - K), (40, 8)),
        line((40, 8), (40, 12)), arc((40, 12), (32 + K, 12 + K), (32, 20)),
        line((32, 20), (28, 20)), arc((28, 20), (28 - K, 28 - K), (20, 28)),  # the concave corner
        line((20, 28), (20, 32)), arc((20, 32), (12 + K, 32 + K), (12, 40)),
        line((12, 40), (8, 40)), arc((8, 40), (8 - K, 32 + K), (0, 32)),
        line((0, 32), (0, 8)), arc((0, 8), (8 - K, 8 - K), (8, 0)),
        line((8, 0), (20, 0)),
    ]
    wire = BRepBuilderAPI_MakeWire()
    for edge in edges:
        wire.Add(edge)
    if not wire.IsDone():
        raise Refused("the rounded outline's edges did not join into a wire")
    return wire.Wire()


def section():
    """The quarter disk, at the path's start (20, 0, 0), where outward is -y: the point (s, z) is
    at (20, -s, z)."""
    k = 6 * math.sqrt(0.5)
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(20, 0, 0), gp_Pnt(20, -6, 0)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(
        gp_Pnt(20, -6, 0), gp_Pnt(20, -k, k), gp_Pnt(20, 0, 6)).Value()).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(20, 0, 6), gp_Pnt(20, 0, 0)).Edge())
    if not wire.IsDone():
        raise Refused("the section's edges did not join into a wire")
    return wire.Wire()


def build():
    pipe = BRepOffsetAPI_MakePipeShell(path())
    pipe.SetMode(gp_Dir(0, 0, 1))  # the section kept upright
    pipe.Add(section())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the section round the outline")
    if not pipe.MakeSolid():
        raise Refused("the swept section did not close into a solid")
    return pipe.Shape()

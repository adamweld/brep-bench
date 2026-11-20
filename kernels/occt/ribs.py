"""What the OCCT implementations of the rib and bead cases share: a planar face from a closed run of
straight and circular edges, its prism and its revolve, and the Booleans."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism, BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Pnt, gp_Vec

from bench import Refused


def face(segments):
    """The planar face inside a closed run of edges. Each segment is (start, end) for a straight
    edge or (start, middle, end) for the circular arc through those three points; points are
    (x, y, z) tuples, and each segment starts where the one before ends."""
    wire = BRepBuilderAPI_MakeWire()
    for seg in segments:
        pts = [gp_Pnt(*p) for p in seg]
        if len(pts) == 2:
            wire.Add(BRepBuilderAPI_MakeEdge(pts[0], pts[1]).Edge())
        else:
            arc = GC_MakeArcOfCircle(pts[0], pts[1], pts[2]).Value()
            wire.Add(BRepBuilderAPI_MakeEdge(arc).Edge())
    return BRepBuilderAPI_MakeFace(wire.Wire()).Face()


def polygon(points):
    """The planar face inside the polygon through `points` (x, y, z tuples), in order."""
    n = len(points)
    return face([(points[i], points[(i + 1) % n]) for i in range(n)])


def segment_xz(cx, cz, r, theta_from, theta_to, y=0):
    """The face in the plane y of a disc of radius r about (cx, cz) cut by the chord between the
    circle's points at angles theta_from and theta_to (degrees, from +x towards +z): the part of
    the disc on the arc's side of the chord."""
    def at(deg):
        a = math.radians(deg)
        return (cx + r * math.cos(a), y, cz + r * math.sin(a))
    return face([(at(theta_from), at((theta_from + theta_to) / 2), at(theta_to)),
                 (at(theta_to), at(theta_from))])


def prism(f, dx, dy, dz):
    return BRepPrimAPI_MakePrism(f, gp_Vec(dx, dy, dz)).Shape()


def revolve(f, degrees):
    """`f` turned about the z axis, from where it lies towards +y (right-handed)."""
    return BRepPrimAPI_MakeRevol(f, gp.OZ_s(), math.radians(degrees)).Shape()


def fuse(a, b, what="the parts"):
    op = BRepAlgoAPI_Fuse(a, b)
    if not op.IsDone():
        raise Refused(f"BRepAlgoAPI_Fuse could not join {what}")
    return op.Shape()


def common(a, b, what="the parts"):
    op = BRepAlgoAPI_Common(a, b)
    if not op.IsDone():
        raise Refused(f"BRepAlgoAPI_Common could not intersect {what}")
    return op.Shape()

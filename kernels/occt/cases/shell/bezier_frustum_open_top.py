"""A frustum lofted between two Bézier loops, shelled with its narrow top open. The solid made by a
ruled loft between two closed curves of four cubic Bézier segments each, (a, 0), (a, ka), (ka, a),
(0, a) and its three quarter turns, at z = 0 with a = 6, k = 0.6 and at z = 8 with a = 3, k = 0.7,
closed by their two flat faces, shelled inward to a wall 1 thick, open at its top face."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt

from bench import Refused
from shell import shell_to_plane

HEIGHT = 8
THICKNESS = 1


def loop(z, a, k):
    # the first segment, and the others its quarter turns about the z axis
    first = [(a, 0), (a, k * a), (k * a, a), (0, a)]
    wire = BRepBuilderAPI_MakeWire()
    for turn in range(4):
        poles = Array1_gp_Pnt(1, 4)
        for i, (x, y) in enumerate(first, start=1):
            for _ in range(turn):
                x, y = -y, x
            poles.SetValue(i, gp_Pnt(x, y, z))
        wire.Add(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge())
    return wire.Wire()


def frustum(top):
    """The loft up to z = `top`: each pole runs straight from its place at z = 0 to its place at
    z = 8 and on past it."""
    f = top / HEIGHT
    # the poles are (a, 0), (a, ka), (ka, a), (0, a): a runs 6 to 3 and ka runs 3.6 to 2.1
    a, ka = 6 + (3 - 6) * f, 3.6 + (2.1 - 3.6) * f
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled between the sections
    loft.AddWire(loop(0, 6, 0.6))
    loft.AddWire(loop(top, a, ka / a))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the Bézier loops")
    return loft.Shape()


def build():
    # the cavity is the solid continued to z = 10, shrunk by 1 and cut off at z = 8
    return shell_to_plane(frustum(HEIGHT), frustum(10), THICKNESS, HEIGHT)

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
from shell import face_through, shell

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


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled between the sections
    loft.AddWire(loop(0, 6, 0.6))
    loft.AddWire(loop(HEIGHT, 3, 0.7))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the Bézier loops")
    solid = loft.Shape()
    return shell(solid, [face_through(solid, (0, 0, HEIGHT))], THICKNESS)

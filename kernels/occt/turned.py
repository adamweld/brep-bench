"""What the OCCT implementations of the offset cases of turned and swept parts share: a profile in
the (ρ, z) half-plane (the plane y = 0, x ≥ 0) turned about the z axis through an angle, and a
circular tube swept along a cubic Bezier spine."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused


def at(rho, z):
    """The point (ρ, 0, z) of the half-plane."""
    return gp_Pnt(rho, 0, z)


def line(a, b):
    """The straight edge between two points."""
    return BRepBuilderAPI_MakeEdge(a, b).Edge()


def turned(edges, angle, what):
    """The solid made by turning the closed outline of `edges`, in order, about the z axis through
    `angle`; Refused, naming `what`, if OCCT makes no solid of it."""
    wire = BRepBuilderAPI_MakeWire()
    for edge in edges:
        wire.Add(edge)
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True)
    if not face.IsDone():
        raise Refused(f"BRepBuilderAPI_MakeFace made no face of {what}")
    solid = BRepPrimAPI_MakeRevol(face.Face(), gp.OZ_s(), angle)
    if not solid.IsDone():
        raise Refused(f"BRepPrimAPI_MakeRevol declined {what}")
    return solid.Shape()


def bezier(points):
    """The cubic Bezier curve through control points (x, y, z)."""
    poles = Array1_gp_Pnt(1, len(points))
    for i, p in enumerate(points):
        poles.SetValue(i + 1, gp_Pnt(*p))
    return Geom_BezierCurve(poles)


def tube(points, radius, what):
    """The solid swept by a disc of `radius`, perpendicular to the cubic Bezier spine through the
    control `points` and centered on it, from one end of the spine to the other."""
    spine = bezier(points)
    start = spine.StartPoint()
    tangent = gp_Dir(*(points[1][i] - points[0][i] for i in range(3)))
    circle = gp_Circ(gp_Ax2(start, tangent), radius)
    rim = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(circle).Edge()).Wire()
    path = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(spine).Edge()).Wire()
    pipe = BRepOffsetAPI_MakePipeShell(path)
    pipe.SetMode(False)  # the corrected Frenet frame, which does not flip at the spine's inflection
    pipe.Add(rim)
    pipe.Build()
    if not pipe.IsDone() or not pipe.MakeSolid():
        raise Refused(f"BRepOffsetAPI_MakePipeShell made no solid of {what}")
    return pipe.Shape()

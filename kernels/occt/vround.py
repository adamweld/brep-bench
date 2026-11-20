"""What the OCCT implementations of the variable-radius round cases share: an edge rounded with a
radius law given at stations along it (BRepFilletAPI_MakeFillet's SetRadius from a table of
parameter and radius), the law written as a function of the point on the edge, so that which way the
edge runs does not matter; and a half-section turned about the z axis."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.collections import Array1_gp_Pnt2d
from OCP.gp import gp, gp_Pnt, gp_Pnt2d

from bench import Refused


def rounded_with_law(solid, edges, radius_at, what, stations=16):
    """`solid` with `edges` (an edge, or a list of edges running on from one another) rounded by a
    radius `radius_at(point)`, set at `stations` + 1 stations spaced evenly along each edge's
    parameter (the ends included); Refused, naming `what`, if OCCT declines. A closed edge's last
    station is its first."""
    if not isinstance(edges, list):
        edges = [edges]
    fillet = BRepFilletAPI_MakeFillet(solid)
    for edge in edges:
        fillet.Add(1.0, edge)
    for contour in range(1, fillet.NbContours() + 1):
        for index in range(1, fillet.NbEdges(contour) + 1):
            curve = BRepAdaptor_Curve(fillet.Edge(contour, index))
            first, last = curve.FirstParameter(), curve.LastParameter()
            table = Array1_gp_Pnt2d(1, stations + 1)
            for k in range(stations + 1):
                t = k / stations
                table.SetValue(k + 1, gp_Pnt2d(t, radius_at(curve.Value(first + (last - first) * t))))
            fillet.SetRadius(table, contour, index)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused(f"BRepFilletAPI_MakeFillet could not round {what} with a varying radius")
    return fillet.Shape()


def angle_about_z(point):
    """The angle of a point about the z axis from +x toward +y."""
    return math.atan2(point.Y(), point.X())


def turned(section, angle=2 * math.pi):
    """The solid made by turning the polygon `section`, corners (ρ, z) in order, in the half-plane
    y = 0, x ≥ 0, about the z axis through `angle` from +x toward +y."""
    polygon = BRepBuilderAPI_MakePolygon()
    for rho, z in section:
        polygon.Add(gp_Pnt(rho, 0, z))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire(), True).Face()
    return BRepPrimAPI_MakeRevol(face, gp.OZ_s(), angle).Shape()

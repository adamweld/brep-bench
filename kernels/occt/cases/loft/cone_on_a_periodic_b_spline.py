r"""A cone on a closed periodic B-spline: a ruled loft from the curve to a point, capped flat.
"""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeVertex, BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.Geom import Geom_BezierCurve, Geom_BSplineCurve
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused


def vertex(p):
    return BRepBuilderAPI_MakeVertex(gp_Pnt(*p)).Vertex()


def cone_loft(wire, apex, name):
    """A ruled ThruSections from the closed wire to the vertex, as a solid (the wire is capped)."""
    thru = BRepOffsetAPI_ThruSections(True, True)
    thru.AddWire(wire)
    thru.AddVertex(vertex(apex))
    thru.Build()
    if not thru.IsDone():
        raise Refused(f"BRepOffsetAPI_ThruSections made no solid for {name}")
    return thru.Shape()


def periodic_spline(poles, z):
    """The closed periodic uniform cubic B-spline of the planar control points, in the plane z,
    knots at the integers, one period per len(poles) knot spans."""
    n = len(poles)
    pts = Array1_gp_Pnt(1, n)
    wts = Array1_double(1, n)
    for i, (x, y) in enumerate(poles, 1):
        pts.SetValue(i, gp_Pnt(x, y, z))
        wts.SetValue(i, 1.0)
    knots = Array1_double(1, n + 1)
    mults = Array1_int(1, n + 1)
    for i in range(n + 1):
        knots.SetValue(i + 1, float(i))
        mults.SetValue(i + 1, 1)
    curve = Geom_BSplineCurve(pts, wts, knots, mults, 3, True)
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(curve).Edge()).Wire()


def build():
    wire = periodic_spline([(4, 0), (1.5, 3), (-3, 2), (-3, -2), (1.5, -3)], 0)
    return cone_loft(wire, (0, 0, 4), "the cone on a spline")

r"""A double cone on a closed Bézier curve. Two cones joined base to base, as a ruled loft from a point through a closed Bézier curve to a second point.

The curve is the cubic Bézier curve in the plane z = 0 with control points (−3, 0, 0), (5, −6, 0), (5, 6, 0) and (−3, 0, 0): its two ends meet at (−3, 0, 0), where it has a corner. It is a teardrop, simple and closed, passing through (3, 0, 0). The two points are (1, 0, 3) above it and (1, 0, −3) below it, over the inside of the teardrop. Each point is joined to every point of the curve by a straight line, so the solid is two cones on the teardrop. The teardrop encloses the area 72/5, so each cone has volume ⅓·(72/5)·3 and the solid 144/5; the area of the cones has no closed form.
"""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeVertex, BRepBuilderAPI_MakeWire)
from OCP.Approx import Approx_ParametrizationType
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.Geom import Geom_BezierCurve, Geom_BSplineCurve
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused


def vertex(p):
    return BRepBuilderAPI_MakeVertex(gp_Pnt(*p)).Vertex()


def circle(radius, z, cx=0.0, cy=0.0):
    ax = gp_Ax2(gp_Pnt(cx, cy, z), gp_Dir(0, 0, 1))
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(gp_Circ(ax, radius)).Edge()).Wire()


def polyline(points):
    poly = BRepBuilderAPI_MakePolygon()
    for p in points:
        poly.Add(gp_Pnt(*p))
    poly.Close()
    return poly.Wire()


def periodic_spline(poles, weights, z):
    """The closed periodic uniform cubic B-spline of the planar control points `poles` in the
    plane z, knots at the integers, one period per len(poles) knot spans."""
    n = len(poles)
    pts = Array1_gp_Pnt(1, n)
    wts = Array1_double(1, n)
    for i, ((x, y), w) in enumerate(zip(poles, weights), 1):
        pts.SetValue(i, gp_Pnt(x, y, z))
        wts.SetValue(i, w)
    knots = Array1_double(1, n + 1)
    mults = Array1_int(1, n + 1)
    for i in range(n + 1):
        knots.SetValue(i + 1, float(i))
        mults.SetValue(i + 1, 1)
    curve = Geom_BSplineCurve(pts, wts, knots, mults, 3, True)
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(curve).Edge()).Wire()


def loft(sections, ruled, name):
    """ThruSections through the vertices and wires, in order, as a solid."""
    thru = BRepOffsetAPI_ThruSections(True, ruled)
    # sections at equal steps of the parameter: the quadratic law of the descriptions (the default
    # chord-length steps bend it when the points are off the axis)
    thru.SetParType(Approx_ParametrizationType.Approx_IsoParametric)
    for s in sections:
        if hasattr(s, "ShapeType") and s.ShapeType().name == "TopAbs_VERTEX":
            thru.AddVertex(s)
        else:
            thru.AddWire(s)
    thru.Build()
    if not thru.IsDone():
        raise Refused(f"BRepOffsetAPI_ThruSections made no solid for {name}")
    return thru.Shape()


def build():
    poles = Array1_gp_Pnt(1, 4)
    for i, p in enumerate([(-3, 0, 0), (5, -6, 0), (5, 6, 0), (-3, 0, 0)], 1):
        poles.SetValue(i, gp_Pnt(*p))
    edge = BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()
    wire = BRepBuilderAPI_MakeWire(edge).Wire()
    return loft([vertex((1, 0, -3)), wire, vertex((1, 0, 3))], True, "the double cone on a Bezier curve")

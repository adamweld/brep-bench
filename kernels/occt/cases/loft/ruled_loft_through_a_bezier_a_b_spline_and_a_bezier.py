r"""A ruled loft through a closed Bézier curve, a periodic B-spline and a closed Bézier curve, between two points. A ruled loft through three stacked closed curves of two kinds, capped by a point at each end, both points displaced sideways.

The curves: $T(t)$ is the cubic Bézier curve with control points (−3, 0), (5, −6), (5, 6), (−3, 0), a teardrop with a corner at (−3, 0), t in [0, 1]; $C(u) = \sum_i P_i N(u - i + 1)$ is the closed periodic cubic B-spline ($N$ the uniform cubic B-spline basis function, the indices running round the five points) with five control points $P_0, \dots, P_4$ = (−3, 2), (−3, −2), (1.5, −3), (4, 0), (1.5, 3), all weights 1, uniform knots at the integers, u from 0 to 5. The sections are $T$ in the plane z = 0, $C$ in the plane z = 3 and $T$ again in the plane z = 6. The end points are (1, 0.5, −3) below the first section and (0, −0.5, 9) above the last. Consecutive curves are joined by straight lines, the point $T(t)$ of a Bézier curve to the point $C(5t)$ of the spline, and each end point to every point of its neighbouring curve.
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


def bezier_wire(poles, z):
    """The closed Bezier curve of the planar control points `poles` in the plane z."""
    pts = Array1_gp_Pnt(1, len(poles))
    for i, (x, y) in enumerate(poles, 1):
        pts.SetValue(i, gp_Pnt(x, y, z))
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(pts)).Edge()).Wire()


def build():
    teardrop = [(-3, 0), (5, -6), (5, 6), (-3, 0)]
    poles = [(-3, 2), (-3, -2), (1.5, -3), (4, 0), (1.5, 3)]
    return loft([vertex((1, 0.5, -3)), bezier_wire(teardrop, 0), periodic_spline(poles, [1] * 5, 3),
                 bezier_wire(teardrop, 6), vertex((0, -0.5, 9))], True, "the mixed stack")

r"""A smooth loft from a point through a periodic B-spline to a point, the points off the axis. A smooth loft from a point through a closed periodic B-spline curve to a second point, both points displaced sideways from the curve's axis.

The curve lies in the plane z = 0. It is the closed periodic cubic B-spline $C(u) = \sum_i P_i N(u - i)$ with five control points $P_0, \dots, P_4$ = (4, 0), (1.5, 3), (−3, 2), (−3, −2), (1.5, −3) and uniform knots at the integers (all weights 1; $N$ the uniform cubic B-spline basis function, the indices running round the five points), closed after one period, u from 0 to 5. The points are $A = (2, 1, -4)$ below it and $B = (-2, 0, 4)$ above it. The loft is the surface traced by one quadratic Bézier curve for each u: its control points are $A$, $2C(u) - (A + B)/2$ and $B$, so it runs from $A$ to $B$ and passes through $C(u)$ at its parameter ½. The solid is bounded by this one surface. Its section at every height is the curve's region scaled about its own centre by a factor $1 - z^2/16$ and moved, so its volume is $\tfrac{16}{15}\cdot 4 \cdot a$, a the area the curve encloses, but its area has no closed form.
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
    poles = [(4, 0), (1.5, 3), (-3, 2), (-3, -2), (1.5, -3)]
    wire = periodic_spline(poles, [1] * 5, 0)
    return loft([vertex((2, 1, -4)), wire, vertex((-2, 0, 4))], False, "the smooth loft")

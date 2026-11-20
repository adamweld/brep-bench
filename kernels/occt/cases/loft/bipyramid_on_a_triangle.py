r"""A bipyramid on a triangle. Two three-sided pyramids joined base to base, as a ruled loft from a point through a triangle to a second point, the two points over the middle of the triangle and at different heights.

The triangle lies in the plane z = 0, with corners (4, 0, 0), (−2, 2√3, 0) and (−2, −2√3, 0): equilateral, side 4√3, centered on the z axis. The two points are (0, 0, 6) above it and (0, 0, −3) below it. Each point is joined to every point of the triangle's boundary by a straight line, so the solid has six flat faces: three triangles rising to the upper point and three falling to the lower one.

**As a set:**

$$\operatorname{conv}\bigl((4,0,0),\ (-2,\pm2\sqrt3,0),\ (0,0,6),\ (0,0,-3)\bigr)$$
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
    s3 = math.sqrt(3)
    triangle = polyline([(4, 0, 0), (-2, 2 * s3, 0), (-2, -2 * s3, 0)])
    # ruled: each point of the boundary is joined to each end point by a straight line
    return loft([vertex((0, 0, -3)), triangle, vertex((0, 0, 6))], True, "the bipyramid")

r"""A smooth loft through a triangle, an octagon and a square. A smooth loft through three stacked polygons of different numbers of sides, closed by the flat faces of the first and the last: a tower that swells from a triangle to an octagon and narrows to a square.

The sections are regular polygons centered on the z axis, in horizontal planes:

| z | polygon | circumradius | a corner at azimuth |
|---|---|---|---|
| −3 | triangle | 2.5 | 90° |
| 0 | octagon | 4 | 0° |
| 3 | square | 3.5 | 45° |

Each section is taken as a closed polyline of 24 stations, $S_j$ for $j = 0 \dots 23$, the station $S_j$ being the point of its boundary at azimuth 15°·j about the z axis (a ray from the axis at that azimuth meets the boundary once); every corner of every polygon is one of its stations. For each station j there is a quadratic Bézier curve $Q_j(t)$ whose control points are $S_j$ of the triangle, $2S_j$ of the octagon less half the sum of $S_j$ of the triangle and of the square, and $S_j$ of the square: it runs from the triangle's station to the square's and passes through the octagon's at its parameter ½. The loft is the surface of the points $(1 - f)\,Q_j(t) + f\,Q_{j+1}(t)$ for j = 0 … 23 (j + 1 taken round), f in [0, 1], t in [0, 1]: at each t the polyline through the points $Q_j(t)$, filled straight between stations. The solid is closed by the flat triangle at the bottom and the flat square at the top.
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


def polygon(n, circumradius, corner_deg, z):
    """The regular n-gon as a closed polyline of 24 stations, one at each azimuth 15 j degrees."""
    step = 2 * math.pi / n
    apothem = circumradius * math.cos(math.pi / n)
    points = []
    for j in range(24):
        phi = math.radians(15 * j)
        local = (phi - math.radians(corner_deg)) % step  # from the corner just before the ray
        r = apothem / math.cos(local - step / 2)
        points.append((r * math.cos(phi), r * math.sin(phi), z))
    return polyline(points)


def build():
    return loft([polygon(3, 2.5, 90, -3), polygon(8, 4.0, 0, 0), polygon(4, 3.5, 45, 3)],
                False, "the smooth stack")

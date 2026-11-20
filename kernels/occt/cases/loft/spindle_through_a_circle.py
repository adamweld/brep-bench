r"""A spindle: a smooth loft from a point through a circle to a point. A smooth loft, not a ruled one, from a point through a circle to a second point, both points on the circle's axis: the sections are joined by a curve, not by a straight line, so that the solid swells from each point to the circle.

The circle has radius 3, lies in the plane z = 0 and is centered on the z axis. The points are (0, 0, 4) and (0, 0, −4). The loft is the surface traced by one quadratic Bézier curve for each point of the circle: for the point $C = (3\cos\theta,\ 3\sin\theta,\ 0)$ the curve has control points $(0, 0, -4)$, $(2C_x,\ 2C_y,\ 0)$ and $(0, 0, 4)$. So it runs from one point to the other and passes through $C$ at its parameter ½. Every such curve is a parabola, ρ = 3(1 − z²/16), and the solid is the one it turns about the z axis: pointed at both ends, its one face a surface of revolution.

**As a set:**

$$\{\,\rho \le 3\,(1 - z^2/16),\ |z| \le 4\,\}$$
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
    # OCCT's smooth ThruSections through a vertex, a circle and a vertex interpolates
    # exactly this law
    return loft([vertex((0, 0, -4)), circle(3, 0), vertex((0, 0, 4))], False, "the spindle")

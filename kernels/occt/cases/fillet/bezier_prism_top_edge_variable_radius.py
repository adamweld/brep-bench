"""A prism on a Bézier-bounded profile with its curved top edge rounded with a growing radius. A prism whose profile in the xy plane is bounded by the straight segment from (0, 0) to (20, 0) and
by the cubic Bézier curve B(t) with control points (0, 0), (4, 14), (16, 14), (20, 0), t from 0 to 1,
which leaves (0, 0) upward and comes back down to (20, 0): a dome 10.5 high at t = 1/2. Extrude it
from z = 0 to z = 10. It has a flat bottom and top, a flat wall on y = 0 and a curved wall standing
on the Bézier curve.

Round the curved edge where the top meets the curved wall, from (0, 0, 10) to (20, 0, 10), with a
radius that grows along it with the curve's parameter: r = 1 + 2t, so 1 where it starts at (0, 0, 10)
and 3 where it ends at (20, 0, 10). The round ends on the flat wall.

The round's sections square to the edge are circular: at each point B(t) of the edge, in the plane
through it square to the edge (the plane that holds the z direction and the curve's normal there),
the round's section is the quarter circle of radius r(t), tangent to the top and to the curved wall,
its center at the point r(t) below the top and r(t) inside the wall from B(t) along the curve's
inward normal. (The curve's radius of curvature is 6.9 at its middle and larger elsewhere, more than
the round's radius, which is at most 3.)"""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierCurve
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused
from vround import rounded_with_law

HEIGHT = 10
POLES = [(0, 0), (4, 14), (16, 14), (20, 0)]


def build():
    poles = Array1_gp_Pnt(1, 4)
    for k, (x, y) in enumerate(POLES):
        poles.SetValue(k + 1, gp_Pnt(x, y, 0))
    bezier = Geom_BezierCurve(poles)
    curve = BRepBuilderAPI_MakeEdge(bezier).Edge()
    base = BRepBuilderAPI_MakeEdge(gp_Pnt(20, 0, 0), gp_Pnt(0, 0, 0)).Edge()
    wire = BRepBuilderAPI_MakeWire(curve)
    wire.Add(base)
    prism = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(wire.Wire()).Face(), gp_Vec(0, 0, HEIGHT)).Shape()

    def parameter_of(point):
        """The Bézier parameter t at the point of the curve with that x, which grows with t."""
        low, high = 0.0, 1.0
        for _ in range(60):
            middle = (low + high) / 2
            if bezier.Value(middle).X() < point.X():
                low = middle
            else:
                high = middle
        return (low + high) / 2

    # The edge on the top that is the Bézier curve.
    top_curve = None
    edges = TopExp_Explorer(prism, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        adaptor = BRepAdaptor_Curve(edge)
        if adaptor.GetType() == GeomAbs_CurveType.GeomAbs_BezierCurve and adaptor.Value(0.5).Z() > HEIGHT - 1e-7:
            top_curve = edge
        edges.Next()
    if top_curve is None:
        raise Refused("the prism has no Bézier edge at its top")
    return rounded_with_law(prism, top_curve, lambda p: 1 + 2 * parameter_of(p), "the curved top edge")

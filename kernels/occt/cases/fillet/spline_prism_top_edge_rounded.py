"""A prism bounded by a cubic B-spline and its chord, the curved top edge rounded. The clamped cubic
B-spline through control points (0, 0), (2, 6), (6, 8), (10, 5), (12, 0), closed by its chord,
extruded to z = 5; the curve's edge at z = 5 rounded to radius 1."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BSplineCurve
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp_Pnt, gp_Vec

from blend import edges_where, rounded

POLES = [(0, 0), (2, 6), (6, 8), (10, 5), (12, 0)]


def build():
    poles = Array1_gp_Pnt(1, len(POLES))
    for i, (x, y) in enumerate(POLES):
        poles.SetValue(i + 1, gp_Pnt(x, y, 0))
    knots = Array1_double(1, 3)
    for i, k in enumerate((0.0, 0.5, 1.0)):
        knots.SetValue(i + 1, k)
    mults = Array1_int(1, 3)
    for i, m in enumerate((4, 1, 4)):
        mults.SetValue(i + 1, m)
    curve = BRepBuilderAPI_MakeEdge(Geom_BSplineCurve(poles, knots, mults, 3)).Edge()
    chord = BRepBuilderAPI_MakeEdge(gp_Pnt(12, 0, 0), gp_Pnt(0, 0, 0)).Edge()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(curve)
    wire.Add(chord)
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 5)).Shape()
    top = edges_where(prism, lambda p: abs(p.Z() - 5) < 1e-7 and p.Y() > 1)
    assert len(top) == 1, len(top)
    return rounded(prism, 1, top, "the curved top edge")

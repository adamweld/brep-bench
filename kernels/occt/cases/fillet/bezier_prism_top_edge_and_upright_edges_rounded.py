"""A prism on a cubic Bézier and its chord with its curved top edge and its two upright edges
rounded, the rounds creased where they meet. The Bézier curve with control points (0, 0), (3, 8),
(9, 9), (12, 0), closed by its chord, extruded from z = 0 to 5. The edge where the curved wall meets
the top, to radius 1; the two upright edges where the wall meets the flat side y = 0, to radius 3/2.
Each round runs the whole length of its edge so that at the top corners they meet in a crease."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt, gp_Vec

from blend import edges_where, rounded_together

POLES = [(0, 0), (3, 8), (9, 9), (12, 0)]
HEIGHT = 5
RIM_RADIUS, UPRIGHT_RADIUS = 1, 1.5


def build():
    poles = Array1_gp_Pnt(1, len(POLES))
    for i, (x, y) in enumerate(POLES):
        poles.SetValue(i + 1, gp_Pnt(x, y, 0))
    curve = BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()
    chord = BRepBuilderAPI_MakeEdge(gp_Pnt(12, 0, 0), gp_Pnt(0, 0, 0)).Edge()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(curve)
    wire.Add(chord)
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, HEIGHT)).Shape()
    top = edges_where(prism, lambda p: abs(p.Z() - HEIGHT) < 1e-7 and p.Y() > 1)
    assert len(top) == 1, len(top)
    uprights = edges_where(prism, lambda p: abs(p.Y()) < 1e-7 and abs(p.Z() - HEIGHT / 2) < 1e-7
                           and (abs(p.X()) < 1e-7 or abs(p.X() - 12) < 1e-7))
    assert len(uprights) == 2, len(uprights)
    return rounded_together(prism, [(RIM_RADIUS, top), (UPRIGHT_RADIUS, uprights)])

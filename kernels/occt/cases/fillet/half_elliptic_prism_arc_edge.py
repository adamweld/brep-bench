"""A half-elliptic prism with its curved top edge rounded. The half-ellipse x²/36 + y²/9 ≤ 1,
y ≥ 0, extruded from z = 0 to z = 4; the arc at z = 4 rounded to radius 1."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt, gp_Vec

from blend import edges_where, rounded


def half_ellipse_prism(a, b, height):
    arc = BRepBuilderAPI_MakeEdge(gp_Elips(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), a, b),
                                  0, math.pi).Edge()
    chord = BRepBuilderAPI_MakeEdge(gp_Pnt(-a, 0, 0), gp_Pnt(a, 0, 0)).Edge()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(arc)
    wire.Add(chord)
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, height)).Shape()


def build():
    prism = half_ellipse_prism(6, 3, 4)
    top_arc = edges_where(prism, lambda p: abs(p.Z() - 4) < 1e-7 and p.Y() > 1)
    assert len(top_arc) == 1, len(top_arc)
    return rounded(prism, 1, top_arc, "the curved top edge")

"""A parabolic-segment prism with its curved top edge rounded. The region 0 ≤ y ≤ 9 − x²/4
extruded from z = 0 to 4; the parabola's edge at z = 4 rounded to radius 1."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Ax2, gp_Dir, gp_Parab, gp_Pnt, gp_Vec

from blend import edges_where, rounded


def build():
    # x² = 4(9 − y): vertex (0, 9), axis down, focal length 1; local (x', y') = (9 − y, x)
    parab = gp_Parab(gp_Ax2(gp_Pnt(0, 9, 0), gp_Dir(0, 0, 1), gp_Dir(0, -1, 0)), 1)
    arc = BRepBuilderAPI_MakeEdge(parab, -6, 6).Edge()
    chord = BRepBuilderAPI_MakeEdge(gp_Pnt(-6, 0, 0), gp_Pnt(6, 0, 0)).Edge()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(arc)
    wire.Add(chord)
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 4)).Shape()
    top = edges_where(prism, lambda p: abs(p.Z() - 4) < 1e-7 and p.Y() > 1)
    assert len(top) == 1, len(top)
    return rounded(prism, 1, top, "the curved top edge")

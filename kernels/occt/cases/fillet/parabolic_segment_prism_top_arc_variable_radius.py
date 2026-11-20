"""A parabolic-segment prism with its curved top edge rounded with a radius that varies along it. The
region 0 ≤ y ≤ 9 − x²/4 extruded from z = 0 to 4; the parabola's edge at z = 4 rounded with
r = 1 − x/12 (3/2 at x = −6, 1/2 at x = 6), the round's section square to the edge the quarter circle
of radius r tangent to the top and the wall."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Ax2, gp_Dir, gp_Parab, gp_Pnt, gp_Vec

from blend import edges_where
from vround import rounded_with_law


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
    # The law is set at stations along the arc, spaced evenly in its parameter x' = x.
    return rounded_with_law(prism, top[0], lambda p: 1 - p.X() / 12, "the curved top edge", stations=48)

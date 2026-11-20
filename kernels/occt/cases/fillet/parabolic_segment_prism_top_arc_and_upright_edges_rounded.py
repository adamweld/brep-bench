"""A parabolic-segment prism with its curved top edge and its two upright edges rounded, the rounds
creased where they meet. The region 0 ≤ y ≤ 9 − x²/4 extruded from z = 0 to 4. The edge where the
parabolic wall meets the top, to radius 1; the two upright edges at (±6, 0) where the wall meets the
flat side y = 0, to radius 3/2. Each round runs the whole length of its edge, ending on the faces at
its ends, so that at the top corners they meet in a crease. Built as one fillet of all three edges, each with its own radius. Finding: one fillet of the three edges closes the corners as the words say (275.489 against 275.486 ± 0.002 from the set); the rounds made one after another do not: the rim first is refused, the upright edges first removes about 2.4 too much (273.07)."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Ax2, gp_Dir, gp_Parab, gp_Pnt, gp_Vec

from blend import edges_where, rounded_together

RIM_RADIUS, UPRIGHT_RADIUS = 1, 1.5


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
    uprights = edges_where(prism, lambda p: abs(abs(p.X()) - 6) < 1e-7 and abs(p.Y()) < 1e-7 and abs(p.Z() - 2) < 1e-7)
    assert len(uprights) == 2, len(uprights)
    return rounded_together(prism, [(RIM_RADIUS, top), (UPRIGHT_RADIUS, uprights)])

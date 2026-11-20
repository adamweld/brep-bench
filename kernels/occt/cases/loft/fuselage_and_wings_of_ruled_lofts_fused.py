"""A fuselage of ruled sections with two wings of ruled sections fused to it. Fuselage: ruled loft
through circles on the x axis at x = 0, 5, 10, 15, 20, radii 2, 4, 5, 4, 2, capped. Wing: ruled loft
through ellipses in the planes y = 0, 7, 14, centres (10, 0, 0), (12, 7, 0), (15, 14, 0), semi-axes
along x and z (4, 0.8), (3, 0.6), (2, 0.4), capped. The fuselage fused with the wing and with the
wing turned 120 degrees about the x axis."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire, BRepBuilderAPI_Transform
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Dir, gp_Elips, gp_Pnt, gp_Trsf

from bench import Refused


def wire(curve):
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(curve).Edge()).Wire()


def ruled(wires):
    loft = BRepOffsetAPI_ThruSections(True, True)  # solid, ruled
    for w in wires:
        loft.AddWire(w)
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the sections")
    return loft.Shape()


def build():
    fuselage = ruled([wire(gp_Circ(gp_Ax2(gp_Pnt(x, 0, 0), gp_Dir(1, 0, 0), gp_Dir(0, 1, 0)), r))
                      for x, r in [(0, 2), (5, 4), (10, 5), (15, 4), (20, 2)]])
    wing = ruled([wire(gp_Elips(gp_Ax2(gp_Pnt(cx, y, 0), gp_Dir(0, 1, 0), gp_Dir(1, 0, 0)), a, b))
                  for cx, y, a, b in [(10, 0, 4, 0.8), (12, 7, 3, 0.6), (15, 14, 2, 0.4)]])
    turn = gp_Trsf()
    turn.SetRotation(gp.OX_s(), 2 * math.pi / 3)
    wing2 = BRepBuilderAPI_Transform(wing, turn, True).Shape()
    first = BRepAlgoAPI_Fuse(fuselage, wing)
    if not first.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not fuse the first wing")
    second = BRepAlgoAPI_Fuse(first.Shape(), wing2)
    if not second.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not fuse the second wing")
    return second.Shape()

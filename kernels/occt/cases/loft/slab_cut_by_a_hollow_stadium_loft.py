"""A slab with a hollow stadium-shaped loft cut through it. Slab [-40, 40] x [-30, 30] x [0, 12].
Stadiums on the x axis: outer, end centres (+-10, 0) radius 8 at z = 0 and (+-14, 0) radius 12 at
z = 12; inner, radius 4 and 7 on the same centres. Ruled lofts, outer to outer, inner to inner; the
tool is the outer less the inner, cut from the slab."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused


def stadium(c, r, z):
    """The stadium with end centres (+-c, 0) and radius r in the plane z, as a wire: the lower
    side, the right end, the upper side, the left end, each from the same starting point."""
    def end(cx, a0, a1):
        return BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(cx, 0, z), gp_Dir(0, 0, 1)), r), a0, a1).Edge()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(-c, -r, z), gp_Pnt(c, -r, z)).Edge())
    wire.Add(end(c, -math.pi / 2, math.pi / 2))
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(c, r, z), gp_Pnt(-c, r, z)).Edge())
    wire.Add(end(-c, math.pi / 2, 3 * math.pi / 2))
    return wire.Wire()


def loft(w0, w1):
    thru = BRepOffsetAPI_ThruSections(True, True)
    thru.AddWire(w0)
    thru.AddWire(w1)
    thru.Build()
    if not thru.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the stadiums")
    return thru.Shape()


def build():
    outer = loft(stadium(10, 8, 0), stadium(14, 12, 12))
    inner = loft(stadium(10, 4, 0), stadium(14, 7, 12))
    tool = BRepAlgoAPI_Cut(outer, inner)
    if not tool.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hollow loft")
    slab = BRepPrimAPI_MakeBox(gp_Pnt(-40, -30, 0), gp_Pnt(40, 30, 12)).Shape()
    cut = BRepAlgoAPI_Cut(slab, tool.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the slab")
    return cut.Shape()

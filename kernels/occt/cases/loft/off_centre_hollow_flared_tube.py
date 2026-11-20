"""A flaring hollow tube with its inner circle off-centre at the foot. Outer circle: radius 20 at
z = 0 to radius 30 at z = 15, centred on the axis. Inner circle: radius 10 centred (2, 0) at z = 0
to radius 18 centred (0, 0) at z = 15. Ruled lofts, the inner frustum cut from the outer."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused


def circle(cx, radius, z):
    edge = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(cx, 0, z), gp_Dir(0, 0, 1)), radius)).Edge()
    return BRepBuilderAPI_MakeWire(edge).Wire()


def frustum(a, b):
    loft = BRepOffsetAPI_ThruSections(True, True)
    loft.AddWire(a)
    loft.AddWire(b)
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the circles")
    return loft.Shape()


def build():
    outer = frustum(circle(0, 20, 0), circle(0, 30, 15))
    inner = frustum(circle(2, 10, 0), circle(0, 18, 15))
    cut = BRepAlgoAPI_Cut(outer, inner)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the inner loft")
    return cut.Shape()

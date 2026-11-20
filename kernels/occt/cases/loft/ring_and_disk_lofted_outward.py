"""A ring and a disk lofted outward together. Circles about the z axis: radii 20, 12, 6 at z = 0
and 30, 18, 9 at z = 10, joined in order by ruled lofts: the ring is the outer frustum less the middle
one, the disk is the inner frustum; together two separate solids."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.TopoDS import TopoDS_Builder, TopoDS_Compound
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused

HEIGHT = 10


def circle(radius, z):
    edge = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), radius)).Edge()
    return BRepBuilderAPI_MakeWire(edge).Wire()


def frustum(r0, r1):
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.AddWire(circle(r0, 0))
    loft.AddWire(circle(r1, HEIGHT))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the circles")
    return loft.Shape()


def build():
    ring = BRepAlgoAPI_Cut(frustum(20, 30), frustum(12, 18))
    if not ring.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the ring")
    compound = TopoDS_Compound()
    builder = TopoDS_Builder()
    builder.MakeCompound(compound)
    builder.Add(compound, ring.Shape())
    builder.Add(compound, frustum(6, 9))
    return compound

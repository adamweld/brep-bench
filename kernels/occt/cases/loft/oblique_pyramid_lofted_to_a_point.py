"""An oblique pyramid lofted from a square to a point. The square of side 6 in z = 0, ruled to the
point (1, 1, 4) as a vertex section."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon, BRepBuilderAPI_MakeVertex
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    poly = BRepBuilderAPI_MakePolygon()
    for x, y in ((3, 3), (-3, 3), (-3, -3), (3, -3)):
        poly.Add(gp_Pnt(x, y, 0))
    poly.Close()
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.AddWire(poly.Wire())
    loft.AddVertex(BRepBuilderAPI_MakeVertex(gp_Pnt(1, 1, 4)).Vertex())
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections declined the square and the point")
    return loft.Shape()

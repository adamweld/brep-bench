"""A square frustum lofted between two squares. The square of side 6 in z = 0 and the concentric
square of side 2 in z = 4, sides parallel, joined by flat ruled faces."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Pnt

from bench import Refused


def square(half, z):
    poly = BRepBuilderAPI_MakePolygon()
    for x, y in ((half, half), (-half, half), (-half, -half), (half, -half)):
        poly.Add(gp_Pnt(x, y, z))
    poly.Close()
    return poly.Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.AddWire(square(3, 0))
    loft.AddWire(square(1, 4))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections declined the two squares")
    return loft.Shape()

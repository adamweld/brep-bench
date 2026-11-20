"""A square lofted to a rectangle in a perpendicular plane. Square (0, 0, 0), (10, 0, 0),
(10, 10, 0), (0, 10, 0); rectangle (30, 2, 30), (30, 2, 10), (30, 8, 10), (30, 8, 30); a ruled loft
joining the matching sides."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Pnt

from bench import Refused


def polygon(points):
    poly = BRepBuilderAPI_MakePolygon()
    for p in points:
        poly.Add(gp_Pnt(*p))
    poly.Close()
    return poly.Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)
    loft.AddWire(polygon([(0, 0, 0), (10, 0, 0), (10, 10, 0), (0, 10, 0)]))
    loft.AddWire(polygon([(30, 2, 30), (30, 2, 10), (30, 8, 10), (30, 8, 30)]))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the square to the rectangle")
    return loft.Shape()

"""A rectangle extruded symmetrically with a taper. The rectangle |x| <= 10, |y| <= 6 in z = 0,
extruded 5 up and 5 down, each side leaning away from the middle 1 in 5: a ruled loft through the
rectangle at z = -5 (22 by 14), at z = 0 (20 by 12) and at z = 5 (22 by 14)."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Pnt

from bench import Refused


def section(half_x, half_y, z):
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in ((-half_x, -half_y), (half_x, -half_y), (half_x, half_y), (-half_x, half_y)):
        polygon.Add(gp_Pnt(x, y, z))
    polygon.Close()
    return polygon.Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.AddWire(section(11, 7, -5))
    loft.AddWire(section(10, 6, 0))
    loft.AddWire(section(11, 7, 5))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the three rectangles")
    return loft.Shape()

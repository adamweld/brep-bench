"""A twisted square frustum shrunk by 0.5. The solid made by a ruled loft between the square of side
4 centered (0, 0, 8), corners (±2, ±2, 8), and the square of side 8 in the plane z = 0 centered
(1, 0, 0) and turned 30° about its center, corners joined by their signs, closed by the two squares,
the whole solid shrunk by 0.5."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Pnt

from bench import Refused
from offset import offset

SIGNS = ((1, 1), (-1, 1), (-1, -1), (1, -1))  # round the square, the same on both


def square(corners):
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y, z in corners:
        polygon.Add(gp_Pnt(x, y, z))
    polygon.Close()
    return polygon.Wire()


def build():
    c, s = math.cos(math.radians(30)), math.sin(math.radians(30))
    upper = [(2 * sx, 2 * sy, 8) for sx, sy in SIGNS]
    lower = [(1 + c * 4 * sx - s * 4 * sy, s * 4 * sx + c * 4 * sy, 0) for sx, sy in SIGNS]
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled between the sections
    loft.AddWire(square(lower))
    loft.AddWire(square(upper))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the twisted frustum")
    return offset(loft.Shape(), -0.5)

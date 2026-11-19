"""A broken edge rounded over. A block from (0, 0, 0) to (10, 10, 5) whose vertical edge at
x = 10, y = 10 is broken by a flat at 45° 0.0001 back along each side: the prism from z = 0 to
z = 5 on (0, 0), (10, 0), (10, 9.9999), (9.9999, 10), (0, 10). Round the two edges where the flat
meets the sides, together, with a fillet of radius 1: the round runs from side to side and the
flat is gone."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from blend import edges_where, rounded

BREAK = 0.0001
SECTION = [(0, 0), (10, 0), (10, 10 - BREAK), (10 - BREAK, 10), (0, 10)]
HEIGHT = 5
RADIUS = 1
TOLERANCE = 1e-7


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in SECTION:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    section = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    block = BRepPrimAPI_MakePrism(section, gp_Vec(0, 0, HEIGHT)).Shape()
    # The flat's two vertical edges, at (10, 9.9999) and (9.9999, 10).
    ends = edges_where(block, lambda p: any(
        abs(p.X() - x) < TOLERANCE and abs(p.Y() - y) < TOLERANCE for x, y in SECTION[2:4]))
    return rounded(block, RADIUS, ends, "the two edges of the broken flat")

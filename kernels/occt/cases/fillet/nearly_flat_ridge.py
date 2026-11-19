"""A ridge nearly flat, rounded by a radius of 180. A prism from z = 0 to z = 6 on the pentagon
(0, 0), (8, 0), (8, 5), (4, 5 + 4 tan 0.5°), (0, 5): a block whose top is two faces meeting at
179° along a ridge at x = 4. Round the ridge with a fillet of radius 180."""
from math import radians, tan

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from blend import edges_where, rounded

PEAK = 5 + 4 * tan(radians(0.5))
SECTION = [(0, 0), (8, 0), (8, 5), (4, PEAK), (0, 5)]
HEIGHT = 6
RADIUS = 180
TOLERANCE = 1e-7


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in SECTION:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    section = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    prism = BRepPrimAPI_MakePrism(section, gp_Vec(0, 0, HEIGHT)).Shape()
    ridge = edges_where(prism, lambda p: abs(p.X() - 4) < TOLERANCE
                        and abs(p.Y() - PEAK) < TOLERANCE)
    return rounded(prism, RADIUS, ridge, "the ridge")

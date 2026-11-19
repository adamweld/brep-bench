"""A knife edge of 5° rounded. A wedge from z = 0 to z = 6 on the triangle (0, 0), (10, 0),
(10, 10 tan 5°), whose edge along the z axis is a knife edge of 5°. Round that edge with a fillet
of radius 0.1."""
from math import radians, tan

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from blend import edges_where, rounded

SECTION = [(0, 0), (10, 0), (10, 10 * tan(radians(5)))]
HEIGHT = 6
RADIUS = 0.1
TOLERANCE = 1e-7


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in SECTION:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    section = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    wedge = BRepPrimAPI_MakePrism(section, gp_Vec(0, 0, HEIGHT)).Shape()
    blade = edges_where(wedge, lambda p: abs(p.X()) < TOLERANCE and abs(p.Y()) < TOLERANCE)
    return rounded(wedge, RADIUS, blade, "the knife edge")

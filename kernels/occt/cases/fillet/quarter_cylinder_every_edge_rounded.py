"""A quarter cylinder with every edge rounded. The points with x >= 0, y >= 0, x² + y² <= 100 and
0 <= z <= 8, all nine edges rounded to radius 1, each corner closed by the rolling ball's sphere."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from blend import edges_where, rounded

RADIUS = 10
HEIGHT = 8
ROUND = 1


def build():
    quarter = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT, math.pi / 2).Shape()
    edges = edges_where(quarter, lambda p: True)
    return rounded(quarter, ROUND, edges, "all the quarter cylinder's edges")

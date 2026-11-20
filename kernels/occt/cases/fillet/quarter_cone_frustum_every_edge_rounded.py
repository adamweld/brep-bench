"""A quarter of a cone frustum with every edge rounded. The points with x >= 0, y >= 0,
0 <= z <= 9 and rho <= 8 - z/3, its nine edges rounded to radius 1."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from blend import edges_where, rounded


def build():
    frustum = BRepPrimAPI_MakeCone(8, 5, 9, math.pi / 2).Shape()
    return rounded(frustum, 1, edges_where(frustum, lambda p: True), "all the frustum's edges")

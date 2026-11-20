"""A quarter of a cone frustum with its top rim rounded. The points with x >= 0, y >= 0,
0 <= z <= 9 and rho <= 8 - z/3. Round the quarter circle of radius 5 where the top meets the cone
to radius 2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from blend import edges_where, rounded

TOLERANCE = 1e-6


def build():
    frustum = BRepPrimAPI_MakeCone(8, 5, 9, math.pi / 2).Shape()
    rim = edges_where(frustum, lambda p: abs(p.Z() - 9) < TOLERANCE and abs(math.hypot(p.X(), p.Y()) - 5) < TOLERANCE)
    return rounded(frustum, 2, rim, "the top rim")

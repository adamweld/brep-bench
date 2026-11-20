"""Three quarters of a cylinder with every convex edge rounded, its inside edge left sharp. The
points with rho <= 10 and 0 <= z <= 8 except x > 0, y < 0; the eight convex edges rounded to
radius 1, the concave edge up the axis left as it is."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from blend import edges_where, rounded

TOLERANCE = 1e-7


def build():
    part = BRepPrimAPI_MakeCylinder(10, 8, 3 * math.pi / 2).Shape()
    convex = edges_where(part, lambda p: math.hypot(p.X(), p.Y()) > TOLERANCE)
    return rounded(part, 1, convex, "the convex edges")

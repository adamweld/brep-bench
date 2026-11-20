"""Three quarters of a cylinder with every convex edge rounded, its inside edge left sharp. The
points with rho <= 10 and 0 <= z <= 8 except x > 0, y < 0; the eight convex edges rounded to
radius 1, the concave edge up the axis left as it is.

Finding: MakeFillet closes the corners with patches of its own, so its solid deviates a little from
the union of balls the case defines (about 6e-4 in volume for the frustum, 8e-5 for the torus patch,
2.4e-5 for the three-quarter cylinder; the zone agrees to 2e-8)."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from blend import edges_where, rounded

TOLERANCE = 1e-7


def build():
    part = BRepPrimAPI_MakeCylinder(10, 8, 3 * math.pi / 2).Shape()
    convex = edges_where(part, lambda p: math.hypot(p.X(), p.Y()) > TOLERANCE)
    return rounded(part, 1, convex, "the convex edges")

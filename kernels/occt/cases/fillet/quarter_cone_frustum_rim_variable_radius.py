"""A quarter of a cone frustum, its top rim rounded with a radius growing along it. The quarter
(x ≥ 0, y ≥ 0) of the frustum on the z axis, radius 10 at z = 0 and 6 at z = 12. Round the quarter
circle where the top meets the cone, from (6, 0, 12) to (0, 6, 12), with r = 3/2 + 2φ/π, φ the angle
about the z axis from +x toward +y: 3/2 at the end face y = 0, 5/2 at the end face x = 0. The
round's sections in the planes through the axis are the circle of radius r(φ) touching the top and
the cone from inside, and the round ends in the two flat end faces."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from chamfer import edge_through
from vround import angle_about_z, rounded_with_law

BOTTOM_RADIUS, TOP_RADIUS, HEIGHT = 10, 6, 12


def build():
    quarter = BRepPrimAPI_MakeCone(BOTTOM_RADIUS, TOP_RADIUS, HEIGHT, math.pi / 2).Shape()
    s = TOP_RADIUS * math.sqrt(0.5)
    rim = edge_through(quarter, (s, s, HEIGHT))  # the quarter circle at the top
    return rounded_with_law(quarter, rim, lambda p: 1.5 + 2 * angle_about_z(p) / math.pi,
                            "the frustum's top rim", stations=32)

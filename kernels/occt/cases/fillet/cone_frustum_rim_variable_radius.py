"""A cone frustum's top rim rounded with a radius that varies all the way round it. A cone frustum on the z axis: radius 10 at its bottom, z = 0, and radius 6 at its top, z = 12. Round
the circular edge around its top face with a radius that varies all the way round it: at the angle φ
about the z axis, measured from +x toward +y, r = 2 − (cos φ)/2. So the radius is 1.5 at φ = 0, where
the edge's seam is, 2.5 opposite it at φ = 180°, and varies smoothly between.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is an arc of the circle of radius r(φ) that touches the top and the cone from
inside, its center at (ρ, z) = (6 − r(√10 − 1)/3, 12 − r). The cone is the line 3ρ + z = 30 in that
plane; the circle touches the top at (6 − r(√10 − 1)/3, 12) and the cone at the center plus
r(3, 1)/√10. The arc between them is the round's section: the corner of the top and the cone, an
angle of about 108.4° there, is cut off by it."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from chamfer import edge_through
from vround import angle_about_z, rounded_with_law

BOTTOM_RADIUS, TOP_RADIUS, HEIGHT = 10, 6, 12


def build():
    frustum = BRepPrimAPI_MakeCone(BOTTOM_RADIUS, TOP_RADIUS, HEIGHT).Shape()
    rim = edge_through(frustum, (0, TOP_RADIUS, HEIGHT))  # the one circle at the top
    return rounded_with_law(frustum, rim, lambda p: 2 - math.cos(angle_about_z(p)) / 2,
                            "the frustum's top rim")

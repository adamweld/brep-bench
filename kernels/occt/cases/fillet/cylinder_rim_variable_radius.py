"""A cylinder's top rim rounded with a radius that varies all the way round it. A cylinder of radius 10 on the z axis, from z = 0 to z = 20. Round the circular edge around its top
face with a radius that varies all the way round it: at the angle φ about the z axis, measured from
+x toward +y, r = 3 − cos φ. So the radius is 2 at φ = 0, where the edge's seam is, 4 opposite it at
φ = 180°, and varies smoothly between, with no kink where it closes.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is the quarter circle of radius r(φ) centered at (ρ, z) = (10 − r, 20 − r),
tangent to the top at ρ = 10 − r and to the side at z = 20 − r."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from chamfer import edge_through
from vround import angle_about_z, rounded_with_law

RADIUS, HEIGHT = 10, 20


def build():
    cylinder = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    rim = edge_through(cylinder, (0, RADIUS, HEIGHT))  # the one circle at the top
    # r = 3 − cos φ, set at stations round the rim; the law is a function of the point's angle,
    # which does not depend on which way the edge runs.
    return rounded_with_law(cylinder, rim, lambda p: 3 - math.cos(angle_about_z(p)),
                            "the cylinder's top rim")

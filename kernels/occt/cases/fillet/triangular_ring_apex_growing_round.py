"""The apex of a triangular ring rounded with a radius growing round a quarter turn. A quarter of a ring with a triangular section: the triangle with corners (ρ, z) = (10, 0), (20, 0)
and (13, 12) in the half-plane y = 0, x > 0, turned about the z axis from +x a quarter of the way
round toward +y. It has a flat bottom ring from ρ = 10 to 20, an inner cone from (10, 0) up to the
apex (13, 12), an outer cone from (20, 0) up to it, and two flat triangular ends in the planes
y = 0 and x = 0. Round the apex, the quarter circle of radius 13 at z = 12 where the two cones meet,
with a radius that grows evenly with the angle φ about the z axis, measured from +x toward +y:
r = 1 + 2φ/π, so 1 at the end in y = 0 and 2 at the end in x = 0.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is the arc of the circle of radius r(φ) inscribed in the triangle's corner at
the apex, tangent to both its sides inside it. (Its center lies on the bisector of the corner, a
distance r/sin(ω/2) from the apex, and it touches each side a distance r/tan(ω/2) from the apex,
where ω is the corner's angle, 44.3°: cos ω = 123/√(153·193).) The round meets each flat end in
such an arc."""
import math

from chamfer import edge_through
from vround import angle_about_z, rounded_with_law, turned

QUARTER = math.pi / 2


def build():
    ring = turned([(10, 0), (20, 0), (13, 12)], QUARTER)
    apex = edge_through(ring, (13 * math.cos(QUARTER / 2), 13 * math.sin(QUARTER / 2), 12))
    # r = 1 + 2φ/π, a function of the point's angle: it does not depend on which way the edge runs.
    return rounded_with_law(ring, apex, lambda p: 1 + angle_about_z(p) / QUARTER,
                            "the ring's apex")

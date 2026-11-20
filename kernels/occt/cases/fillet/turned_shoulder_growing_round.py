"""A turned shoulder's inside corner rounded with a radius growing round a quarter turn. A quarter of a shaft with a shoulder, the part with x ≥ 0 and y ≥ 0: a cylinder of radius 10 on the z
axis from z = 0 to z = 10, and on it a cylinder of radius 5 from z = 10 to z = 30. The big
cylinder's top, the ring from ρ = 5 to 10 at z = 10, is the shoulder. Round the inside corner where
the shoulder meets the small cylinder, the quarter circle of radius 5 at z = 10 from (5, 0, 10) to
(0, 5, 10), with a radius that grows evenly with the angle φ about the z axis, measured from +x
toward +y: r = 1 + 4φ/π, so 1 where the edge meets the flat side y = 0 and 3 where it meets the flat
side x = 0. The round adds material: it fills the corner.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is the quarter circle of radius r(φ) centered at (ρ, z) = (5 + r, 10 + r),
tangent to the shoulder at ρ = 5 + r and to the small cylinder at z = 10 + r; the round fills the
square corner between them out to that quarter circle. The round meets each flat side in such a
quarter circle, of radius 1 on y = 0 and 3 on x = 0."""
import math

from chamfer import edge_through
from vround import angle_about_z, rounded_with_law, turned

QUARTER = math.pi / 2


def build():
    # The half-section of the shaft, turned a quarter of the way round from +x toward +y.
    shaft = turned([(0, 0), (10, 0), (10, 10), (5, 10), (5, 30), (0, 30)], QUARTER)
    corner = edge_through(shaft, (5 * math.cos(QUARTER / 2), 5 * math.sin(QUARTER / 2), 10))
    # r = 1 + 4φ/π, a function of the point's angle: it does not depend on which way the edge runs.
    return rounded_with_law(shaft, corner, lambda p: 1 + angle_about_z(p) / QUARTER * 2,
                            "the shoulder's inside corner")

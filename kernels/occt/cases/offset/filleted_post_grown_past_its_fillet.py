"""A post on a plate, filleted at its foot, grown past the fillet's radius. A plate, the cylinder of
radius 8 on the z axis from z = −4 to z = 0, with a post standing on it, the cylinder of radius 3
from z = 0 to z = 8. The concave edge where the post meets the plate, the circle of radius 3 at z
= 0, is filleted to radius 2: a quarter of a torus whose tube has radius 2 about the circle of
radius 5 at z = 2, concave, tangent to the post's side at z = 2 and to the plate's top at radius
5. Grow the solid by 3: every point within 3 of it.

That is more than the fillet's radius. The fillet moved 3 outward would be a torus of tube radius 2
− 3 = −1, turned inside out: it lies inside the grown solid and is no part of its boundary. The
post's side, moved out to radius 6, and the plate's top, moved up to z = 3, run on past it and meet
in a sharp concave edge, the circle of radius 6 at z = 3. The fillet makes no difference to the
result: it is the plate and post without it, grown by 3.

The convex edges are rounded, as a growth rounds them: the plate's two rims by quarter tori of tube
radius 3 about the circles of radius 8 at z = −4 and z = 0, and the post's top edge by one about the
circle of radius 3 at z = 8. The grown solid runs from z = −7 to z = 11 and out to radius 11."""
from offset import revolved

# The grown solid's half-section in (ρ, z), counter-clockwise from the axis: a corner to run
# straight to, or an arc about a center, of a radius, from one angle to another in degrees.
SECTION = [
    ('to', 0, -7),
    ('to', 8, -7),
    ('arc', 8, -4, 3, 270, 360),
    ('to', 11, 0),
    ('arc', 8, 0, 3, 0, 90),
    ('to', 6, 3),
    ('to', 6, 8),
    ('arc', 3, 8, 3, 0, 90),
    ('to', 0, 11),
]


def build():
    # BRepOffsetAPI_MakeOffsetShape makes no solid of this growth: it does not drop the fillet's
    # moved face, turned inside out. The result is built as the case states it: its half-section,
    # the moved faces and the rounds of the convex edges, turned about the z axis.
    return revolved(SECTION, "the grown solid's half-section")

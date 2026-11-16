"""A block with a blind bore, filleted at the bore's floor, grown past the fillet's radius. A block,
the cylinder of radius 10 on the z axis from z = 0 to z = 8, with a blind bore in its top: the
cylinder of radius 5 from z = 2 up through the top is cut away, leaving a flat floor at z = 2.
The concave edge where the bore's side meets its floor, the circle of radius 5 at z = 2, is
filleted to radius 2: a quarter of a torus whose tube has radius 2 about the circle of radius 3
at z = 4, concave, tangent to the floor at radius 3 and to the bore's side at z = 4. Grow the
solid by 3: every point within 3 of it.

That is more than the fillet's radius. The fillet moved 3 outward, into the bore, would be a torus
of tube radius 2 − 3 = −1, turned inside out: it lies inside the grown solid and is no part of its
boundary. The bore's side, moved in to radius 2, and its floor, moved up to z = 5, run on past it
and meet in a sharp concave edge, the circle of radius 2 at z = 5. The fillet makes no difference to
the result: it is the bored block without it, grown by 3.

The convex edges are rounded, as a growth rounds them: the block's two rims by quarter tori of tube
radius 3 about the circles of radius 10 at z = 0 and z = 8, and the bore's mouth by one about the
circle of radius 5 at z = 8, which narrows the bore from radius 5 at the grown top, z = 11, to
radius 2 at z = 8. The grown solid runs from z = −3 to z = 11 and out to radius 13."""
from offset import revolved

# The grown solid's half-section in (ρ, z), counter-clockwise from the axis: a corner to run
# straight to, or an arc about a center, of a radius, from one angle to another in degrees.
SECTION = [
    ('to', 0, -3),
    ('to', 10, -3),
    ('arc', 10, 0, 3, 270, 360),
    ('to', 13, 8),
    ('arc', 10, 8, 3, 0, 90),
    ('to', 5, 11),
    ('arc', 5, 8, 3, 90, 180),
    ('to', 2, 5),
    ('to', 0, 5),
]


def build():
    # BRepOffsetAPI_MakeOffsetShape makes no solid of this growth: it does not drop the fillet's
    # moved face, turned inside out. The result is built as the case states it: its half-section,
    # the moved faces and the rounds of the convex edges, turned about the z axis.
    return revolved(SECTION, "the grown solid's half-section")

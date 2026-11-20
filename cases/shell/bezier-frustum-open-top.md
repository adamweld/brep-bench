+++
title = "A frustum lofted between two Bézier loops, shelled with its narrow top open"
tags = ["shell", "inward", "loft", "bezier", "ruled"]
origin = "OCCT tests/bugs/modalg_2/bug5805_42"
settled = false
+++

A solid made by a ruled loft between two closed curves of square-like shape, closed by the two flat
faces they bound. Each curve is four cubic Bézier segments, for a size a and a looseness k:

| segment | from | control | control | to |
|---|---|---|---|---|
| 1 | (a, 0) | (a, ka) | (ka, a) | (0, a) |
| 2 | (0, a) | (−ka, a) | (−a, ka) | (−a, 0) |
| 3 | (−a, 0) | (−a, −ka) | (−ka, −a) | (0, −a) |
| 4 | (0, −a) | (ka, −a) | (a, −ka) | (a, 0) |

It is smooth everywhere: a square with its corners bulged out toward (±a, ±a), the more the
larger k. The lower curve lies in the plane z = 0 with a = 6, k = 0.6; the upper curve lies in the
plane z = 8 with a = 3, k = 0.7. The loft joins the point at parameter t of each segment of the
lower curve to the point at the same t of the same segment of the upper curve by a straight line;
the side is the ruled surface made so.

Shell it inward to a wall 1 thick, open at its top face, z = 8. The cavity is every point of the
solid at least 1 from its side and at least 1 above its bottom, running up to the top plane, which
is the opening; the wall is everything else in the solid. The result is the solid less that cavity.

The side is not a quadric and its inward offset has no closed form, so no volume or area is
stated.

**As a set:**

$$S \setminus \{\, p \in S : d(p,\ \partial S \cap \{z < 8\}) \ge 1 \,\}, \quad S \text{ the lofted solid}$$

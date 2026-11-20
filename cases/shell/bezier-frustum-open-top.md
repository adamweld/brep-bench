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
the side is the ruled surface made so. Each pole runs in a straight line from its place at z = 0
to its place at z = 8.

Shell it inward to a wall 1 thick, open at its top face, z = 8. The side's moved face runs on past
the plane of the opening and is cut by it: the side is taken as continued beyond z = 8, and the
rim is where its offset meets the plane z = 8. Exactly: let E be the solid continued up to z = 10
by the same loft, each pole's line run on past z = 8 to z = 10: there the poles are those of the segments above with a = 2.25 for the
on-axis poles and 1.725 in place of ka, and the curve there closes it. The cavity is E shrunk by 1, every point of E at
least 1 from E's boundary (its side, its floor and its far end), cut off at the plane z = 8; the
far end is more than 1 above it. The result is the solid less that cavity.

The side is not a quadric and its inward offset has no closed form, so no volume or area is
stated.

**As a set:**

$$S \setminus \bigl((E \ominus B_1) \cap \{z \le 8\}\bigr), \quad E \text{ the solid continued to } z = 10,\ S = E \cap \{z \le 8\}$$

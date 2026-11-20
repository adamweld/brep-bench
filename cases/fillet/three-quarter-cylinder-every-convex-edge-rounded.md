+++
title = "Three quarters of a cylinder with every convex edge rounded, its inside edge left sharp"
tags = ["fillet", "corner", "concave", "cylinder", "sphere", "torus", "planar"]
settled = false
origin = "OCCT blend/simple/B5"
+++

Three quarters of a cylinder of radius 10: the points with ρ ≤ 10 and 0 ≤ z ≤ 8, except those with
x > 0 and y < 0, where ρ = √(x² + y²). The two radial faces, y = 0 for x ≥ 0 and x = 0 for y ≤ 0, make
a re-entrant edge up the axis, from (0, 0, 0) to (0, 0, 8): a concave edge, which stays sharp. Its
other eight edges are convex: the three-quarter circles of radius 10 around the bottom and the top; the
four straight ones where the bottom and the top meet the radial faces; and the two upright ones where
the radial faces meet the curved wall, at (10, 0) and (0, −10). Round those eight to radius 1.

The solid is the union of every ball of radius 1 that fits inside the part: the rolling ball closes
each corner with a patch of its own, and leaves the concave edge as it was.

**As a set:**

$$(K \ominus B_1) \oplus B_1,\qquad K = \{\, \rho \le 10,\ 0 \le z \le 8 \,\} \setminus \{\, x > 0,\ y < 0 \,\}$$

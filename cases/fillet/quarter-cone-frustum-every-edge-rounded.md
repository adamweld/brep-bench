+++
title = "A quarter of a cone frustum with every edge rounded"
tags = ["fillet", "corner", "cone", "sphere", "torus", "planar"]
settled = false
origin = "OCCT blend/simple/B9"
+++

A quarter turn of a cone frustum: the points with x ≥ 0, y ≥ 0, 0 ≤ z ≤ 9 and
ρ ≤ 8 − z/3, where ρ = √(x² + y²). It has nine edges: the one up the axis, from (0, 0, 0) to
(0, 0, 9); the four straight ones where the flat bottom and the flat top meet the radial faces x = 0
and y = 0; the quarter circles of radius 8 and 5 around the bottom and the top; and the two straight
ones where the cone meets the radial faces. Round all nine to radius 1.

Three rounds meet at each of the eight corners, and each corner is closed by a patch of the sphere of
radius 1 that touches the three faces there. The solid is the union of every ball of radius 1 that
fits inside the frustum's quarter.

**As a set:**

$$(K \ominus B_1) \oplus B_1,\qquad K = \{\, x \ge 0,\ y \ge 0,\ 0 \le z \le 9,\ \rho \le 8 - z/3 \,\}$$

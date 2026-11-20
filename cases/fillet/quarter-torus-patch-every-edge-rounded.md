+++
title = "A quarter turn of a torus's tube, cut flat, with every edge rounded"
tags = ["fillet", "corner", "torus", "planar"]
settled = false
origin = "OCCT blend/simple/E6"
+++

A quarter turn about the z axis of a torus's tube cut flat: the points with x ≥ 0, y ≥ 0,
(ρ − 6)² + z² ≤ 4 and z ≤ 1, where ρ = √(x² + y²). It is bounded by the torus (ring radius 6, tube
radius 2), by the flat top z = 1, a quarter of an annulus from ρ = 6 − √3 to ρ = 6 + √3, and by the
two radial faces x = 0 and y = 0. It has six edges: the quarter circles of radius 6 − √3 and 6 + √3
where the top meets the torus; the two straight ones where the top meets the radial faces; and the two
arcs of the tube's circle, radius 2, where the torus meets the radial faces. Round all six to radius
1/2.

Three rounds meet at each of the four corners, and each corner is closed by a patch of the sphere of
radius 1/2 that touches the three faces there. The solid is the union of every ball of radius 1/2
that fits inside the patch.

**As a set:**

$$(K \ominus B_{1/2}) \oplus B_{1/2},\qquad K = \{\, x \ge 0,\ y \ge 0,\ (\rho-6)^2 + z^2 \le 4,\ z \le 1 \,\}$$

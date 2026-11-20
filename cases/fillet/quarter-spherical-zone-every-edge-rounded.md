+++
title = "A quarter of a spherical zone with every edge rounded"
tags = ["fillet", "corner", "sphere", "torus", "planar"]
settled = false
origin = "OCCT blend/simple/D3"
+++

A quarter turn of a band of a ball: the points with x ≥ 0, y ≥ 0, 2 ≤ z ≤ 7 and
x² + y² + z² ≤ 100. It has nine edges: the one up the axis, from (0, 0, 2) to (0, 0, 7); the four
straight ones where the flat bottom (z = 2) and the flat top (z = 7) meet the radial faces x = 0 and
y = 0; the quarter circles of radius √96 and √51 around the bottom and the top; and the two half-circle
arcs of radius 10, in the planes x = 0 and y = 0, where the sphere meets the radial faces, between
z = 2 and z = 7. Round all nine to radius 1.

Three rounds meet at each of the eight corners, and each corner is closed by a patch of the sphere of
radius 1 that touches the three faces there. The solid is the union of every ball of radius 1 that
fits inside the quarter zone.

**As a set:**

$$(K \ominus B_1) \oplus B_1,\qquad K = \{\, x \ge 0,\ y \ge 0,\ 2 \le z \le 7,\ \|p\| \le 10 \,\}$$

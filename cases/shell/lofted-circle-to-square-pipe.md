+++
title = "A pipe lofted from a circle to a square, open at both ends"
tags = ["shell", "inward", "loft", "ruled", "freeform"]
origin = "OCCT tests/offset/bugs/bug5806"
settled = false
+++

A solid lofted between two sections along the z axis: the circle of radius 3 about the axis in the
plane z = 0, and the boundary of the square |x| ≤ 4, |y| ≤ 4 in the plane z = 10. The loft is ruled,
and the sections are paired side by side: the circle's quarter arc from 45° to 135° with the square's
side y = 4, running from the corner (4, 4) to the corner (−4, 4), and likewise the arcs from 135° to
225°, 225° to 315° and 315° to 45° with the sides x = −4, y = −4 and x = 4, each running to the next
corner. Within a pair the points correspond in proportion: the point of the arc at 45° + 90°·s is
joined by a straight line to the point of the side at the fraction s of its length from its first
corner, (4 − 8s, 4, 10), for s from 0 to 1 (the point at 45° to the corner (4, 4), the point at 135° to
(−4, 4)). So its side is made of the lines so drawn, four ruled faces that meet in four sharp edges,
the lines from the circle's points at 45°, 135°, 225° and 315° to the square's corners. Its ends are
the flat disk in z = 0 and the flat square in z = 10.

Shell it inward to a wall 1/2 thick, open at both ends, a pipe. The outside of the wall is the loft's
side; the inside is the set of points of the loft at least 1/2 from that side, which runs on from
z = 0 to z = 10 to meet the planes of the openings, so the rims are flat in those planes. The side
is not developable and the inner surface is its parallel surface, with no closed form, so the case
is unsettled until two kernels agree.

**As a set:**

$$S \setminus \bigl(S \ominus_{\text{side}} B_{1/2}\bigr)$$

where S is the solid whose cross-section in the plane z = 10t is the region bounded by the closed curve of the points (1 − t)·(3 cos θ, 3 sin θ) + t·(4 − 8s, 4), θ = 45° + 90°s, s from 0 to 1, and its three quarter turns about the axis, and the second set is the points of S at least 1/2 from S's side, the ends not counted.

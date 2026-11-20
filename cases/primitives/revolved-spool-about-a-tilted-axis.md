+++
title = "A spool turned about a tilted axis beside its profile"
tags = ["primitive", "revolve", "analytic", "cone", "tilted-axis"]
origin = "OCCT modalg_1/bug15036"

[volume]
value = 2153.373268476588
exact = "2π·A·d̄ = 2π·36·(238/25) = 17136π/25: the profile's area is 36 and its centroid (35/3, 6) is d̄ = (24·35/3 − 7·6)/25 = 238/25 from the axis"
tex = '2\pi A\,\bar d = 2\pi\cdot 36\cdot\tfrac{238}{25} = \tfrac{17136\pi}{25}'

[area]
value = 2177.5007000561577
exact = "2π·Σ Lᵢ·dᵢ over the eight edges (length, distance of the midpoint from the axis): (4, 288/25), (3, 651/50), (2, 291/25), (6, 246/25), (2, 249/25), (3, 21/2), (4, 204/25), (12, 198/25); the sum is 8664/25, so 17328π/25. Every edge is slanted to the axis, so every face is a cone"
tex = '2\pi\sum_i L_i d_i = \tfrac{17328\pi}{25}'
+++

In the xz plane, take the eight-sided profile of a spool, a narrow neck between two wide flanges,
with corners in order (10, 0), (14, 0), (14, 3), (12, 3), (12, 9), (14, 9), (14, 12), (10, 12): a
flange 4 wide and 3 tall at each end, and a neck 2 wide between them. Turn it a whole turn about
the line through the origin in the direction (7, 0, 24)/25, a line in the profile's plane tilted a
little from the z axis (about 16°), which passes beside the profile and never touches it: the
nearest point of the profile to the line is the corner (10, 12), 6.24 from it.

No side of the profile is parallel to the axis or square to it, so each of the eight faces is a
cone. The solid has no flat faces and no cylinders.

**As a set:** with $s = (7x + 24z)/25$ and $t = \sqrt{x^2 + y^2 + z^2 - s^2}$, the distances along the axis and from it, the solid is the points where $(s, t)$ lies in the polygon with the corners $\bigl((7x_i + 24z_i)/25,\ (24x_i - 7z_i)/25\bigr)$ for the eight $(x_i, z_i)$ above.

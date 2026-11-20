+++
title = "A cap of a ball shrunk by 1"
tags = ["offset", "shrink", "sphere", "analytic"]
origin = "OCCT tests/offset/shape_type_i/D5"

[volume]
value = 385.368698840348
exact = "πh²(3R − h)/3, R = 9, h = 4"
tex = '\tfrac{1}{3}\pi h^2 (3R - h),\quad R = 9,\ h = 4'

[area]
value = 402.1238596594935
exact = "2πRh + π(R² − 5²) = 72π + 56π: the sphere's zone and the disk of radius 2√14"
tex = '2\pi R h + \pi (R^2 - 5^2),\quad R = 9,\ h = 4'
+++

The part of the ball of radius 10 about the origin that lies below the plane z = −4: a spherical cap
from the south pole (0, 0, −10) up to the circle of radius 8 at z = −4, closed by a flat disk of
radius 8 in that plane. Its two faces meet at that circle in a convex edge.

Shrink it by 1: the solid of every point at least 1 from the cap's outside.

The sphere moves in to the sphere of radius 9 about the origin, the disk moves in, into the cap,
down to z = −5, and they meet along the circle of radius 2√14 at z = −5. The result is the part of
the ball of radius 9 below the plane z = −5.

**As a set:**

$$\{\, \lVert p \rVert \le 9,\ z \le -5 \,\}$$

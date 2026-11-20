+++
title = "A cap of a ball grown by 1, its edges kept sharp"
tags = ["offset", "grow", "sharp-edge", "sphere", "analytic"]
origin = "OCCT tests/offset/shape_type_i/D4"

[volume]
value = 1675.5160819145565
exact = "πh²(3R − h)/3, R = 11, h = 8: a cap of a ball of radius 11, 8 deep"
tex = '\tfrac{1}{3}\pi h^2 (3R - h),\quad R = 11,\ h = 8'

[area]
value = 904.7786842338604
exact = "2πRh + π(R² − 3²) = 176π + 112π: the sphere's zone and the disk of radius 4√7"
tex = '2\pi R h + \pi (R^2 - 3^2),\quad R = 11,\ h = 8'
+++

The part of the ball of radius 10 about the origin that lies below the plane z = −4: a spherical cap
from the south pole (0, 0, −10) up to the circle of radius 8 at z = −4, closed by a flat disk of
radius 8 in that plane. Its two faces meet at that circle in a convex edge.

Grow it by 1 with its edges kept sharp: move each face 1 outward along its normal, and extend the
moved faces until they meet, so that no round is added and every edge stays where its two faces
meet.

The sphere moves out to the sphere of radius 11 about the origin, the disk moves out, away from the
cap, up to z = −3, and they meet along the circle of radius 4√7 at z = −3. The result is the part of
the ball of radius 11 below the plane z = −3.

**As a set:**

$$B_{11} \cap \{\, z \le -3 \,\}$$

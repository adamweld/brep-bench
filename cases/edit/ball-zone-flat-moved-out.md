+++
title = "A ball's zone from its south pole, its flat moved out"
tags = ["edit", "move-face", "sphere", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/G3"

[volume]
value = 3934.3211998456177
exact = "π(R²(a + R) − (a³ + R³)/3), R = 10, a = 7: the ball's part below the plane z = a"
tex = '\pi\left(R^2(a+R) - \tfrac{a^3 + R^3}{3}\right) = \tfrac{3757}{3}\pi,\quad R = 10,\ a = 7'

[area]
value = 1228.3627275536091
exact = "2πR(a + R) + π(R² − a²) = 340π + 51π = 391π: the zone and the flat disk"
tex = '2\pi R(a+R) + \pi\left(R^2 - a^2\right) = 391\pi'
+++

The part of the ball of radius 10 centered on the origin that lies below the plane z = 3, from its south pole, (0, 0, −10), up to that latitude. Its faces are the spherical zone and one flat disk of radius √91 in the plane z = 3, facing up.

Then edit the solid directly: move the flat 4 outward along its normal, up to the plane z = 7. The sphere runs on round above the old rim to meet it, so the flat becomes a disk of radius √51.

So the result is the ball less everything above z = 7: its zone from the south pole to the latitude z = 7, and a flat disk of radius √51 closing it.

**As a set:**

$$B_{10} \cap \{\, z \le 7 \,\}$$

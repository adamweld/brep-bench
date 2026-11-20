+++
title = "A ball's zone from its south pole, its flat moved in"
tags = ["edit", "move-face", "sphere", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/G4"

[volume]
value = 1781.2830345854127
exact = "π(R²(a + R) − (a³ + R³)/3), R = 10, a = −1: the ball's part below the plane z = a"
tex = '\pi\left(R^2(a+R) - \tfrac{a^3 + R^3}{3}\right) = 567\pi,\quad R = 10,\ a = -1'

[area]
value = 876.5043503515523
exact = "2πR(a + R) + π(R² − a²) = 180π + 99π = 279π: the zone and the flat disk"
tex = '2\pi R(a+R) + \pi\left(R^2 - a^2\right) = 279\pi'
+++

The part of the ball of radius 10 centered on the origin that lies below the plane z = 3, from its south pole, (0, 0, −10), up to that latitude. Its faces are the spherical zone and one flat disk of radius √91 in the plane z = 3, facing up.

Then edit the solid directly: move the flat 4 inward along its normal, down to the plane z = −1. The sphere is trimmed back to meet it, so the flat becomes a disk of radius √99.

So the result is the ball less everything above z = −1: its zone from the south pole to the latitude z = −1, and a flat disk of radius √99 closing it.

**As a set:**

$$B_{10} \cap \{\, z \le -1 \,\}$$

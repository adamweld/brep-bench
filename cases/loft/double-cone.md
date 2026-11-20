+++
title = "A double cone"
tags = ["loft", "ruled", "point-ends", "cone", "analytic"]
origin = "OCCT thrusection/specific/A9"

[volume]
value = 75.398223686155038
exact = "2·(1/3)·πr²h with r = 3, h = 4"
tex = '2\cdot\tfrac13\pi\cdot 3^2\cdot 4 = 24\pi'

[area]
value = 94.247779607693797
exact = "two cones of lateral area πr·s, slant s = √(3² + 4²) = 5, so 2·π·3·5"
tex = '2\cdot\pi\cdot3\cdot5 = 30\pi'
+++

Two cones joined base to base, as a ruled loft from a point through a circle to a second point, both points on the circle's axis.

The circle has radius 3, lies in the plane z = 0 and is centered on the z axis. The two points are (0, 0, 4) and (0, 0, −4). Each point is joined to every point of the circle by a straight line. The solid has two conical faces, and no flat face.

**As a set:**

$$\{\,\rho \le 3\,(1 - |z|/4),\ |z| \le 4\,\}$$

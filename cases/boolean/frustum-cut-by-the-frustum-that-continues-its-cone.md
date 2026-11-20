+++
title = "A cone frustum cut by the frustum that continues its cone"
tags = ["boolean", "cut", "analytic", "coincident", "tangent", "cone", "touching"]
origin = "OCCT tests/boolean/bopcut_simple/ZL6"

[volume]
value = 795.8701389094143
exact = "πh(a² + ab + b²)/3 with h = 10, a = 6, b = 4; nothing is removed"
tex = '\tfrac{\pi h}{3}\left(a^2 + ab + b^2\right),\quad h = 10,\ a = 6,\ b = 4'

[area]
value = 483.7436628749511
exact = "πa² + πb² + π(a + b)·√(h² + (a − b)²) = 52π + 20√26·π"
tex = '\pi a^2 + \pi b^2 + \pi(a + b)\sqrt{h^2 + (a - b)^2} = 52\pi + 20\sqrt{26}\,\pi'
+++

A cone frustum on the z axis from z = 0 to z = 10: radius 6 at its base, radius 4 at its top. And a second frustum on the same axis from z = 10 to z = 20: radius 4 at its base, radius 2 at its top. Together they are one cone, 6 across at z = 0 and losing a fifth of a unit of radius per unit of height all the way to z = 20; the second begins where the first ends. Cut the second frustum away from the first.

The two touch along the circle of radius 4 in the plane z = 10 and over the disc that circle bounds, and nothing else, and their side surfaces are the same cone, so they meet without a crease. The result is the first frustum, unchanged.

**As a set:**

$$\{\rho \le 6 - \tfrac15 z,\ 0 \le z \le 10\} \setminus \{\rho \le 6 - \tfrac15 z,\ 10 \le z \le 20\}$$

+++
title = "A frustum with a second frustum under it, widening up to meet its base circle"
tags = ["boolean", "union", "analytic", "cone", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZM7"

[volume]
value = 153.93804002589988
exact = "πh/3·(R² + Rr + r²) for each: 95π/3 (h = 5, 3 → 2) and 52π/3 (h = 4, 1 → 3)"
tex = '\tfrac{95\pi}{3} + \tfrac{52\pi}{3}'

[area]
value = 152.00169233834524
exact = "4π + π·5·√26 + π·4·2√5 + π: the upper frustum's top and wall, the lower one's wall and bottom; the shared disc of radius 3 is interior"
tex = '4\pi + 5\sqrt{26}\,\pi + 8\sqrt5\,\pi + \pi'
+++

A frustum of a cone on the z axis, from z = 0 to z = 5: radius 3 at z = 0, narrowing in a straight line to radius 2 at z = 5.

A second frustum, coaxial with it, below it: from z = −4 to z = 0, radius 1 at z = −4, widening in a straight line to radius 3 at z = 0. Its wide end is exactly the first frustum's base disc. Make the union of the two.

The union is a spindle: the two cones meet along the circle of radius 3 at z = 0 in a crease, since the lower one is widening upward while the upper one narrows.

**As a set:**

$$\{\,0 \le z \le 5,\ \rho \le 3 - \tfrac15 z\,\} \cup \{\,-4 \le z \le 0,\ \rho \le 3 + \tfrac12 z\,\}$$

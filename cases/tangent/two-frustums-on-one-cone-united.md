+++
title = "Two frustums cut from one cone, stacked and united"
tags = ["tangent", "boolean", "union", "analytic", "cone", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZL6"

[volume]
value = 435.63418129778466
exact = "πh/3·(R² + Rr + r²) of the whole frustum, h = 8, R = 6, r = 2"
tex = '\tfrac{8\pi}{3}(36 + 12 + 4)'

[area]
value = 350.457777536895
exact = "36π + π(R + r)·√(h² + (R − r)²) + 4π: the whole frustum's base, wall (8π·4√5) and top"
tex = '36\pi + 32\sqrt5\,\pi + 4\pi'
+++

A frustum of a cone on the z axis, from z = 0 to z = 4: radius 6 at z = 0, narrowing in a straight line to radius 4 at z = 4.

A second frustum, coaxial with it, standing on its top face: from z = 4 to z = 8, radius 4 at z = 4, narrowing in a straight line to radius 2 at z = 8. The two are slices of one cone (the line through (6, 0), (4, 4) and (2, 8) is straight), and the second's base is the first's top disc exactly. Make the union of the two.

The union is the single frustum from z = 0 to z = 8, radius 6 narrowing to radius 2, whose conical side is continuous across z = 4 with no edge there.

**As a set:**

$$\{\,0 \le z \le 4,\ \rho \le 6 - \tfrac12 z\,\} \cup \{\,4 \le z \le 8,\ \rho \le 6 - \tfrac12 z\,\}$$

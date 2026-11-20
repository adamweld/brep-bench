+++
title = "A frustum united with a narrower frustum hidden inside it, as tall as it is"
tags = ["boolean", "union", "analytic", "cone", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZM3"

[volume]
value = 245.04422698000388
exact = "78π: the union is the larger frustum, πh/3·(R² + Rr + r²), h = 6, R = 5, r = 2"
tex = '\tfrac{6\pi}{3}(25 + 10 + 4)'

[area]
value = 238.62729630595925
exact = "25π + 4π + π(R + r)·√(h² + (R − r)²): its two ends and its wall, 7π·3√5"
tex = '25\pi + 4\pi + 21\sqrt5\,\pi'
+++

A frustum of a cone on the z axis, from z = 0 to z = 6: radius 5 at z = 0, narrowing in a straight line to radius 2 at z = 6.

A second, narrower frustum, coaxial with it and as tall as it: from z = 0 to z = 6, radius 3 at z = 0, narrowing in a straight line to radius 1 at z = 6. Make the union of the two.

The second frustum's base lies in the first one's base and its top in the first one's top face, both wholly inside their outlines (radius 3 inside radius 5, radius 1 inside radius 2); its cone surface is clear of the first's. Everything of it lies inside the first frustum or on its surface, so the union is the first frustum itself.

**As a set:**

$$\{\,0 \le z \le 6,\ \rho \le 5 - \tfrac12 z\,\} \cup \{\,0 \le z \le 6,\ \rho \le 3 - \tfrac13 z\,\}$$

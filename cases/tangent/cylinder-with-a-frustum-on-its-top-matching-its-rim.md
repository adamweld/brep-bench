+++
title = "A cylinder with a frustum on its top whose base is the cylinder's own rim"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "cone", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZK1"

[volume]
value = 368.61353802120243
exact = "80π + πh/3·(R² + Rr + r²), R = 4, r = 2, h = 4"
tex = '80\pi + \tfrac{4\pi}{3}(16 + 8 + 4)'

[area]
value = 272.79333598787633
exact = "16π + 40π + π(R + r)·√(h² + (R − r)²) + π·r²: the cylinder's bottom and wall, the frustum's wall (6π·2√5) and its top; the cylinder's top face and the frustum's base are gone"
tex = '16\pi + 40\pi + 12\pi\sqrt5 + 4\pi'
+++

A cylinder of radius 4, its axis the z axis, from z = 0 to z = 5.

A frustum of a cone, coaxial with it, standing on its top face: from z = 5 to z = 9, radius 4 at z = 5, the same as the cylinder's, narrowing in a straight line to radius 2 at z = 9. Make the union of the two.

The frustum's base circle is exactly the cylinder's top rim, so the two have a whole face in common: the cylinder's top disc is the frustum's base. The union is a drum with a conical cap, the wall changing from cylinder to cone along the rim in a crease.

**As a set:**

$$\{\,\rho \le 4,\ 0 \le z \le 5\,\} \cup \{\,5 \le z \le 9,\ \rho \le 4 - \tfrac12 (z - 5)\,\}$$

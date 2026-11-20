+++
title = "A cylinder with a narrower frustum standing on its top"
tags = ["boolean", "union", "analytic", "cylinder", "cone"]
origin = "OCCT tests/boolean/bopfuse_simple/ZJ4"

[volume]
value = 525.6931707006921
exact = "150π + πh/3·(R² + Rr + r²), h = 4, R = 3, r = 1: the cylinder and the frustum"
tex = '150\pi + \tfrac{4\pi}{3}(9 + 3 + 1)'

[area]
value = 376.6409685144847
exact = "25π + 60π + 16π + π(R + r)·√(h² + (R − r)²) + π·r²: the cylinder's bottom, wall and top ring (25π − 9π), the frustum's wall (4π·2√5) and its top"
tex = '25\pi + 60\pi + 16\pi + 4\pi\sqrt{20} + \pi'
+++

A cylinder of radius 5, its axis the z axis, from z = 0 to z = 6.

A frustum of a cone, coaxial with it, standing on its top face: from z = 6 to z = 10, radius 3 at z = 6 narrowing in a straight line to radius 1 at z = 10. Make the union of the two.

The frustum's base lies in the cylinder's top face, wholly inside its outline, so the union is the cylinder with the frustum on a step of 2 (from radius 5 to radius 3).

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 6\,\} \cup \{\,6 \le z \le 10,\ \rho \le 3 - \tfrac12 (z - 6)\,\}$$

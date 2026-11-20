+++
title = "A cylinder united with a frustum that comes out below its base and ends flush with its top"
tags = ["boolean", "union", "analytic", "cylinder", "cone", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZJ7"

[volume]
value = 697.2008585300015
exact = "200π + πh/3·(r₀² + r₀r₁ + r₁²) for the stub below the base: h = 4, r₀ = 2, r₁ = 8/3"
tex = '200\pi + \tfrac{4\pi}{3}\bigl(4 + \tfrac{16}{3} + \tfrac{64}{9}\bigr)'

[area]
value = 458.08517206706625
exact = "25π (top) + 80π (wall) + (25 − 64/9)π (the base's ring) + π(r₀ + r₁)·√(h² + (r₁ − r₀)²) (the stub's wall) + 4π (its bottom)"
tex = '25\pi + 80\pi + \bigl(25 - \tfrac{64}{9}\bigr)\pi + \tfrac{28\sqrt{37}}{9}\pi + 4\pi'
+++

A cylinder of radius 5, its axis the z axis, from z = 0 to z = 8.

A frustum of a cone, coaxial with it, from z = −4 to z = 8: radius 2 at z = −4, widening in a straight line to radius 4 at z = 8. So it comes out 4 below the cylinder's base, and its top face lies in the cylinder's top face plane. Make the union of the two.

Above z = 0 the frustum lies inside the cylinder, its cone surface well clear of the cylinder's wall, and only its top disc (radius 4) touches the cylinder's boundary, in the cylinder's top face. Below z = 0 the frustum stands out of the cylinder's base: the cone surface crosses the base plane in a circle of radius 8/3.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 8\,\} \cup \{\,-4 \le z \le 8,\ \rho \le 2 + \tfrac16 (z + 4)\,\}$$

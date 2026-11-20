+++
title = "A cylinder and a frustum wholly inside it, their common part"
tags = ["boolean", "common", "analytic", "cone", "contained"]
origin = "OCCT tests/boolean/bopcommon_simple/ZJ8"

[volume]
value = 81.681408993334624
exact = "πh(a² + ab + b²)/3 with h = 6, a = 3, b = 1: the common part is the frustum itself"
tex = '\tfrac{\pi h}{3}\left(a^2+ab+b^2\right),\quad h=6,\ a=3,\ b=1'

[area]
value = 110.89263266226674
exact = "π(a + b)√(h² + (a − b)²) + π(a² + b²)"
tex = '\pi(a+b)\sqrt{h^2+(a-b)^2} + \pi(a^2+b^2)'
+++

A cylinder of radius 5 and height 10 standing on the xy plane, its axis the z axis, and a cone frustum on the same axis from z = 2 to z = 8: radius 3 at its lower end, radius 1 at its upper end. Take the part common to the cylinder and the frustum.

The frustum lies wholly inside the cylinder and touches none of its faces, so the common part is the frustum itself: a truncated cone 6 high with a flat disc of radius 3 below and one of radius 1 above.

**As a set:**

$$\{\rho \le 5,\ 0 \le z \le 10\} \cap \{\rho \le 3 - \tfrac{z-2}{3},\ 2 \le z \le 8\}$$

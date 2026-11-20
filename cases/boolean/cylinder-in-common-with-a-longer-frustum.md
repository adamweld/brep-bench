+++
title = "A cylinder and a frustum through it from end to end, their common part"
tags = ["boolean", "common", "analytic", "cone"]
origin = "OCCT tests/boolean/bopcommon_simple/ZJ9"

[volume]
value = 288.08618347204362
exact = "πh(a² + ab + b²)/3 with h = 10, a = 16/7, b = 26/7 (the frustum's radii at z = 0 and z = 10)"
tex = '\tfrac{\pi h}{3}\left(a^2+ab+b^2\right),\quad h=10,\ a=\tfrac{16}{7},\ b=\tfrac{26}{7}'

[area]
value = 250.16364333220931
exact = "π(a + b)√(h² + (a − b)²) + π(a² + b²)"
tex = '\pi(a+b)\sqrt{h^2+(a-b)^2} + \pi(a^2+b^2)'
+++

A cylinder of radius 5 and height 10 standing on the xy plane, its axis the z axis, and a cone frustum on the same axis from z = −2 to z = 12: radius 2 at its lower end, radius 4 at its upper end. Take the part common to the cylinder and the frustum.

The frustum is narrower than the cylinder throughout and longer than it at both ends, so the cylinder's wall touches nothing: only its two end planes cut the frustum. The common part is the stretch of the frustum from z = 0 to z = 10, a truncated cone of radius 16/7 at its base and 26/7 at its top.

**As a set:**

$$\{\rho \le 5,\ 0 \le z \le 10\} \cap \{\rho \le 2 + \tfrac{z+2}{7},\ -2 \le z \le 12\}$$

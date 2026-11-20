+++
title = "A cone and a cylinder of its base radius on its base disc, their common part"
tags = ["boolean", "common", "analytic", "cone", "coincident"]
origin = "OCCT tests/boolean/bopcommon_simple/ZP7"

[volume]
value = 274.88935718910691
exact = "πh(a² + ab + b²)/3 with h = 6, a = 5, b = 5/2: the lower part of the cone, up to z = 6"
tex = '\tfrac{\pi h}{3}\left(a^2+ab+b^2\right),\quad h=6,\ a=5,\ b=\tfrac52'

[area]
value = 251.32741228718346
exact = "π(a + b)√(h² + (a − b)²) + π(a² + b²) = 80π"
tex = '\pi(a+b)\sqrt{h^2+(a-b)^2} + \pi(a^2+b^2) = 80\pi'
+++

A cone with its apex at (0, 0, 12) and its base the disc of radius 5 in the plane z = 0 about the origin, and a cylinder of radius 5 and height 6 on the same axis standing on the same base disc, from z = 0 to z = 6. Take the part common to the two.

The two base discs coincide, and the cone's base circle is the cylinder's base circle. Above it the cone narrows, so it lies inside the cylinder and the cylinder's wall touches it only along that circle. The common part is the lower half of the cone, a truncated cone from z = 0 to z = 6 with radius 5 at its base and 5/2 at its top, where the cylinder's top plane cuts it.

**As a set:**

$$\{\rho \le 5 - \tfrac{5z}{12},\ 0 \le z \le 12\} \cap \{\rho \le 5,\ 0 \le z \le 6\}$$

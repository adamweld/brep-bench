+++
title = "A frustum and a narrower frustum of its height, level with it at both ends, their common part"
tags = ["boolean", "common", "analytic", "cone", "coincident"]
origin = "OCCT tests/boolean/bopcommon_simple/ZM3"

[volume]
value = 293.21531433504737
exact = "πh(a² + ab + b²)/3 with h = 10, a = 4, b = 2: the common part is the narrower frustum itself"
tex = '\tfrac{\pi h}{3}\left(a^2+ab+b^2\right),\quad h=10,\ a=4,\ b=2'

[area]
value = 255.06036000476499
exact = "π(a + b)√(h² + (a − b)²) + π(a² + b²)"
tex = '\pi(a+b)\sqrt{h^2+(a-b)^2} + \pi(a^2+b^2)'
+++

A cone frustum on the z axis from z = 0 to z = 10, radius 6 at its base and 4 at its top, and a second frustum on the same axis over the same span, radius 4 at its base and 2 at its top. Take the part common to the two.

The second frustum is narrower than the first at every height, so it lies inside it; their base discs lie in one plane and so do their top discs, the second's within the first's. The common part is the second frustum, a truncated cone 10 high.

**As a set:**

$$\{\rho \le 6 - \tfrac{z}{5},\ 0 \le z \le 10\} \cap \{\rho \le 4 - \tfrac{z}{5},\ 0 \le z \le 10\}$$

+++
title = "A cube united with a cone frustum standing on its top"
tags = ["boolean", "union", "analytic", "cone", "coincident", "face-contact"]
origin = "OCCT tests/boolean/bopfuse_simple/ZF5"

[volume]
value = 237.99114857512856
exact = "6³ + (π·3/3)(2² + 2·1 + 1²) = 216 + 7π"
tex = '216 + \tfrac{\pi\cdot 3}{3}\,(4 + 2 + 1) = 216 + 7\pi'

[area]
value = 236.37898683661894
exact = "area = (216 − 4π) (the cube, its top less the frustum's base disc) + π(2 + 1)·√(3² + 1²) (the cone's wall) + π (the frustum's top)"
tex = '216 - 4\pi + 3\pi\sqrt{10} + \pi'
+++

A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum standing on its top face: axis the z axis, base radius 2 at z = 6, top radius 1 at z = 9. Make their union.

The two touch only over the frustum's base disc, which lies wholly inside the cube's top face; nothing overlaps. The result is the cube with a cone frustum standing on it.

**As a set:**

$$[-3,3]\times[-3,3]\times[0,6] \cup \{\rho \le 2 - (z-6)/3,\ 6 \le z \le 9\}$$

+++
title = "A post united with a narrower post sunk partway into its top and standing out of it"
tags = ["boolean", "union", "analytic", "cylinder", "face-contact"]
origin = "OCCT tests/boolean/bopfuse_simple/ZC9"

[volume]
value = 898.4954989266809
exact = "π·5²·10 + π·3²·4 = 286π: the post, and the narrow cylinder's part above z = 10"
tex = '25\pi\cdot 10 + 9\pi\cdot 4 = 286\pi'

[area]
value = 546.637121724624
exact = "area = 25π (post's base) + 2π·5·10 (post's wall) + (25 − 9)π (post's top, a ring) + 2π·3·4 (the narrow cylinder's wall above z = 10) + 9π (its top) = 174π"
tex = '25\pi + 100\pi + 16\pi + 24\pi + 9\pi = 174\pi'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A second cylinder of radius 3 with the same axis, from z = 6 to z = 14: it is sunk 4 into the post and stands 4 out of its top. Make the union of the two.

The narrow cylinder's wall crosses the post's top face square, along the circle ρ = 3, z = 10. Below it the narrow cylinder is inside the post and goes; its base, at z = 6, is inside the post too.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \{\,\rho \le 3,\ 6 \le z \le 14\,\}$$

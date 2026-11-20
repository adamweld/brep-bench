+++
title = "A post in common with a ball centered on its top disc"
tags = ["boolean", "common", "sphere", "cylinder", "hemisphere"]
origin = "OCCT tests/boolean/bopcommon_simple/ZI4, OCCT tests/boolean/bopcommon_simple/ZI5, OCCT tests/boolean/bopcommon_simple/ZI6, OCCT tests/boolean/bopcommon_simple/ZI7"

[volume]
value = 16.755160819145564
exact = "half a ball of radius 2: (2/3)π·2³"
tex = '\tfrac{2}{3}\pi\cdot 2^3 = \tfrac{16\pi}{3}'

[area]
value = 37.69911184307752
exact = "the hemisphere 2π·2² = 8π and the flat disc π·2² = 4π"
tex = '2\pi\cdot 2^2 + \pi\cdot 2^2 = 12\pi'
+++

A post: a solid cylinder of radius 4 standing on the xy plane, its axis the z axis, from z = 0 to
z = 10. A ball of radius 2 centered on the middle of the post's top disc, the point (0, 0, 10). The
ball's lower half is inside the post, well clear of its wall; its upper half is above the post.
Make the part common to both: a hemisphere of radius 2, with its flat face in the plane z = 10, where
the sphere crosses the post's top disc in a circle of radius 2.

**As a set:**

$$\{\,\rho \le 4,\ 0 \le z \le 10\,\} \cap B_2\bigl((0, 0, 10)\bigr)$$

+++
title = "A drilled block's rounded hole rim deleted: the top and the bore meet again"
tags = ["edit", "delete-face", "fillet", "torus", "cylinder", "hole", "analytic"]

[volume]
value = 1874.3362938564082
exact = "20·10·10 − π·2²·10 = 2000 − 40π"
tex = '20\cdot 10\cdot 10 - \pi\cdot 2^2\cdot 10 = 2000 - 40\pi'

[area]
value = 1100.5309649148735
exact = "2(20·10 + 20·10 + 10·10) − 2·π·2² + 2π·2·10 = 1000 + 32π"
tex = '2(20\cdot 10 + 20\cdot 10 + 10\cdot 10) - 2\cdot\pi\cdot 2^2 + 2\pi\cdot 2\cdot 10 = 1000 + 32\pi'
+++

A block, the box from the origin to (20, 10, 10), drilled through from top to bottom by a hole of
radius 2 about the vertical line x = 10, y = 5. The edge where the hole's wall meets the top, z =
10, is rounded to radius 1: the round is a quarter of a torus whose tube has radius 1 about the
circle of radius 3 at z = 9, centered on the hole's axis, tangent to the top along the circle of
radius 3 and to the hole's wall along the circle at z = 9.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The top
runs on inward from radius 3 and the hole's wall up from z = 9 until they meet in the circle of
radius 2 at z = 10.

The result is the block drilled through by the hole of radius 2, its rim sharp.

**As a set:**

$$[0,20]\times[0,10]\times[0,10] \setminus \{\,(x-10)^2 + (y-5)^2 < 2^2\,\}$$

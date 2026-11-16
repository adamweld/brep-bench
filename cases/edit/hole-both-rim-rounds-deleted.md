+++
title = "A drilled block's two rounded hole rims deleted: the faces and the bore meet again at both ends"
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
radius 2 about the vertical line x = 10, y = 5, with both of the hole's rims rounded to radius 1:
at the top a quarter of a torus whose tube has radius 1 about the circle of radius 3 at z = 9, and
at the bottom its mirror image about the circle of radius 3 at z = 1, each centered on the hole's
axis and tangent to its face and to the hole's wall.

Then edit it directly: delete both rounds together. Deleting faces takes them out of the solid and
closes the gaps they leave with the faces round them, each run on along its own surface until they
meet. The top and the bottom run on inward from radius 3, and the wall up from z = 9 and down from z
= 1, until they meet in the circles of radius 2 at z = 10 and at z = 0.

The result is the block drilled through by the hole of radius 2, both its rims sharp.

**As a set:**

$$[0,20]\times[0,10]\times[0,10] \setminus \{\,(x-10)^2 + (y-5)^2 < 2^2\,\}$$

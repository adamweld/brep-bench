+++
title = "A blind bore's beveled floor deleted: the floor and the wall meet again"
tags = ["edit", "delete-face", "chamfer", "cone", "cylinder", "hole", "analytic"]

[volume]
value = 3698.40710525538
exact = "20·20·10 − π·4²·6 = 4000 − 96π"
tex = '20\cdot 20\cdot 10 - \pi\cdot 4^2\cdot 6 = 4000 - 96\pi'

[area]
value = 1750.79644737231
exact = "2(20·20 + 20·10 + 20·10) − π·4² + 2π·4·6 + π·4² = 1600 + 48π"
tex = '2(20\cdot 20 + 20\cdot 10 + 20\cdot 10) - \pi\cdot 4^2 + 2\pi\cdot 4\cdot 6 + \pi\cdot 4^2 = 1600 + 48\pi'
+++

A block, the box from (−10, −10, 0) to (10, 10, 10), with a blind bore of radius 4 about the z axis sunk from its
top down to a floor at z = 4; but the concave edge where the floor meets the bore's wall is beveled: the wall
stops at z = 5 and a cone runs from there down and in to the floor, now the disk of radius 3 at z = 4. The
cutter is the cylinder of radius 4 from z = 5 up through the top joined to the frustum from radius 3 at z = 4
to radius 4 at z = 5.

Then edit it directly: delete the bevel. Deleting a face takes it out of the solid and closes the gap it
leaves with the faces round it, each run on along its own surface until they meet. The floor runs on outward
from radius 3 and the wall down from z = 5 until they meet in the circle of radius 4 at z = 4.

The result is the block with the sharp-floored bore.

**As a set:**

$$[-10,10]^2\times[0,10] \setminus \{\,x^2 + y^2 < 16,\ z > 4\,\}$$

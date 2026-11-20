+++
title = "A cylinder cut in half by a block tangent to it on three sides"
tags = ["boolean", "cut", "analytic", "tangent", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/R8"

[volume]
value = 113.09733552923255
exact = "half a cylinder: ½·π·3²·8"
tex = '\tfrac12\pi r^2 h,\quad r = 3,\ h = 8'

[area]
value = 151.67255756846316
exact = "half the side 2π·3·8/2, two half disks π·3², and the flat face 6 × 8"
tex = '\pi r h + \pi r^2 + 2 r h'
+++

A cylinder of radius 3 and height 8 standing on the xy plane with its axis the z axis, and a block from (−3, −3, 0) to (0, 3, 8), as tall as the cylinder, its base and its top in the cylinder's. The block's faces x = −3, y = −3 and y = 3 are tangent to the cylinder, along the vertical lines through (−3, 0), (0, −3) and (0, 3); its fourth face, x = 0, passes through the axis. The block's corners (0, ±3) are on the cylinder's wall and its corners (−3, ±3) are outside the cylinder. Cut the block away from the cylinder: what is left is the half of the cylinder in x ≥ 0.

**As a set:**

$$\{\rho \le 3,\ 0 \le z \le 8\} \setminus \bigl([-3,0]\times[-3,3]\times[0,8]\bigr)$$

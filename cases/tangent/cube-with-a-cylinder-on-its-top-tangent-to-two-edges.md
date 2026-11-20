+++
title = "A cube with a small cylinder standing on its top, tangent to two adjacent top edges"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopfuse_simple/ZO9"

[volume]
value = 1050.2654824574367
exact = "1000 + π·2²·4"
tex = '1000 + \pi\cdot 2^2\cdot 4'

[area]
value = 650.2654824574367
exact = "600 + 2π·2·4: the cube's area (the cylinder's end disc replaces the disc it covers on the top) plus the cylinder's wall"
tex = '600 + 2\pi\cdot 2\cdot 4'
+++

A cube with x, y and z from 0 to 10.

A cylinder of radius 2, its axis the vertical line through (2, 2), from z = 10 to z = 14, standing on the cube's top face. Its base circle touches the top face's edges x = 0 and y = 0, each at one point, (0, 2, 10) and (2, 0, 10). Make the union of the two.

**As a set:**

$$[0,10]^3 \cup \{\,(x-2)^2 + (y-2)^2 \le 2^2,\ 10 \le z \le 14\,\}$$

+++
title = "A post capped by a ball of its own radius centred on its top"
tags = ["boolean", "union", "analytic", "cylinder", "sphere", "tangent-line"]
origin = "OCCT tests/boolean/bopfuse_simple/ZH5"

[volume]
value = 1047.1975511965977
exact = "π·5²·10 + ½·(4π/3)·5³ = 1000π/3: the post and the ball's upper half"
tex = '25\pi\cdot 10 + \tfrac23\pi\cdot 5^3 = \tfrac{1000\pi}{3}'

[area]
value = 549.7787143782139
exact = "area = 25π (post's base) + 2π·5·10 (wall) + 2π·5² (the ball's upper half); the post's top, a disc inside the ball, goes"
tex = '25\pi + 100\pi + 50\pi = 175\pi'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A ball of radius 5 centred on the middle of the post's top face, (0, 0, 10). Make the union of the post and the ball.

The ball's equator is the post's top rim; the ball's surface meets the post's wall tangentially all round that circle, so the two run on smoothly into one another there. Below z = 10 the ball is inside the post and goes. What is left is the post with a hemispherical dome.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup B_5\bigl((0,0,10)\bigr)$$

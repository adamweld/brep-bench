+++
title = "A post and a ball tangent to its wall along its top rim, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "sphere"]
origin = "OCCT tests/boolean/bopcommon_simple/ZH5"

[volume]
value = 261.79938779914943
exact = "2πr³/3, r = 5"
tex = '\tfrac{2}{3}\pi r^3,\quad r = 5'

[area]
value = 235.61944901923448
exact = "2πr² + πr²"
tex = '2\pi r^2 + \pi r^2'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A ball of radius 5 centred at (0, 0, 10), the middle of the post's top disc. Take the part common to the two.

The ball's equator is the post's top rim, and there the ball's surface is tangent to the post's wall, all the way round. Below the rim the ball lies inside the post, which is as tall as the ball's radius twice over, so its base is never reached.

What is left is the lower half of the ball: a hemisphere with its flat face the post's top disc.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap B_5\bigl((0,0,10)\bigr)$$

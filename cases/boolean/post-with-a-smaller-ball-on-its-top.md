+++
title = "A post united with a smaller ball centred on its top"
tags = ["boolean", "union", "analytic", "cylinder", "sphere", "face-contact"]
origin = "OCCT tests/boolean/bopfuse_simple/ZI4"

[volume]
value = 841.9468311620645
exact = "π·5²·10 + ½·(4π/3)·3³ = 268π: the post and the ball's upper half"
tex = '25\pi\cdot 10 + \tfrac23\pi\cdot 3^3 = 268\pi'

[area]
value = 499.5132319207771
exact = "area = 25π (post's base) + 2π·5·10 (wall) + (25 − 9)π (the top, a ring) + 2π·3² (the ball's upper half) = 159π"
tex = '25\pi + 100\pi + 16\pi + 18\pi = 159\pi'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A ball of radius 3 centred on the middle of the post's top face, (0, 0, 10). Make the union of the post and the ball.

The ball's equator is a circle inside the post's top face, and its lower half is inside the post and goes. What is left is the post with a hemispherical bump on its top, meeting the top face along that circle at a right angle.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup B_3\bigl((0,0,10)\bigr)$$

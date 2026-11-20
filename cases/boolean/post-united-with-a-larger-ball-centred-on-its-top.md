+++
title = "A post united with a larger ball centred on its top, its wall cutting the ball's surface"
tags = ["boolean", "union", "analytic", "cylinder", "sphere"]
origin = "OCCT tests/boolean/bopfuse_simple/ZI8"

[volume]
value = 678.5840131753953
exact = "π·3²·10 + (4π/3)·5³ − C = 216π, C = 36π + 14π/3 the ball's part inside the post (z from 5 to 10: the section is the post's disc 9π where the ball is wider, for z − 10 from −4 to 0, and the ball's own π(25 − (z − 10)²) below it)"
tex = '9\pi\cdot 10 + \tfrac{500\pi}{3} - \left(36\pi + \tfrac{14\pi}{3}\right) = 216\pi'

[area]
value = 424.1150082346221
exact = "area = 9π (post's base) + 2π·3·6 (the wall below z = 6, where it meets the ball) + (100π − 2·10π) + 10π (the sphere's zone outside the post, ρ > 3, and its top cap, ρ ≤ 3, each cap of height 1 and area 2π·5·1) = 135π; the post's top and the rest of its wall are inside the ball"
tex = '9\pi + 36\pi + 80\pi + 10\pi = 135\pi'
+++

A post: a solid cylinder of radius 3 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A ball of radius 5 centred on the middle of the post's top face, (0, 0, 10). Make the union of the post and the ball.

The ball is wider than the post: it stands 5 above the top face, and its lower part, down to z = 5, surrounds the post's top. The ball's surface cuts the post's wall in the circle ρ = 3, z = 6. Above that circle, the post's wall and top are inside the ball; below it the wall runs on to the base, at z = 0, clear of the ball.

**As a set:**

$$\{\,\rho \le 3,\ 0 \le z \le 10\,\} \cup B_5\bigl((0,0,10)\bigr)$$

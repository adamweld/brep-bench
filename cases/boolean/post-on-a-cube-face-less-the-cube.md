+++
title = "A post centred on a cube's face, less the cube"
tags = ["boolean", "cut", "analytic", "cylinder", "tangent", "coincident"]
origin = "OCCT tests/boolean/boptuc_simple/ZN5"

[volume]
value = 392.69908169872415
exact = "half of π·5²·10"
tex = '\tfrac12\pi\cdot 5^2\cdot 10'

[area]
value = 335.61944901923449
exact = "half wall π·5·10, the flat 10 × 10, and two half-discs 2 · ½π·5²"
tex = '50\pi + 100 + 25\pi'
+++

A cube of side 10 between the origin and (10, 10, 10).

A post of radius 5 and height 10, from z = 0 to z = 10, standing on the cube's bottom plane and flush with its top plane, whose axis is the vertical line through (5, 0): it lies on the middle of the cube's side face y = 0, with its wall tangent to the two neighbouring faces x = 0 and x = 10. Cut the cube away from the post.

What is left is the half of the post that sticks out of the cube, in y < 0: a half-round bar 10 high with a flat back 10 by 10 on the plane y = 0.

**As a set:**

$$\{\,(x - 5)^2 + y^2 \le 25,\ 0 \le z \le 10\,\} \setminus [0, 10]^3$$

+++
title = "A cube with a half-round notch tangent to its two neighbouring sides"
tags = ["boolean", "cut", "analytic", "tangent", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/ZN5"

[volume]
value = 607.3009183012758
exact = "10³ − ½·π·5²·10"
tex = '10^3 - \tfrac12\pi\cdot 5^2\cdot 10'

[area]
value = 578.5398163397448
exact = "top and bottom 2(10² − ½π·5²), the faces x = 0 and x = 10 whole (2·100), the face y = 10 (100), the half wall π·5·10"
tex = '2\left(10^2 - \tfrac12\pi 5^2\right) + 200 + 100 + \pi\cdot 5\cdot 10 = 500 + 25\pi'
+++

A cube of side 10 between the origin and (10, 10, 10), less a cylinder of radius 5 and height 10 whose axis is the vertical line through (5, 0), from z = 0 to z = 10: the middle of the side face y = 0, the cylinder's ends in the cube's top and bottom. Half the cylinder is inside the cube, the half with y ≥ 0. The cylinder is tangent to the cube's faces x = 0 and x = 10, along the vertical lines through (0, 0) and (10, 0), which are the cube's own vertical edges.

The face y = 0 is cut away whole, and the half-round notch it leaves runs the full height of the cube. Where the notch meets the faces x = 0 and x = 10 it meets them tangentially, at their edges, so each of those faces and the notch's wall form a cusp of angle zero there.

**As a set:**

$$[0,10]^3 \setminus \{(x-5)^2 + y^2 \le 5^2\}$$

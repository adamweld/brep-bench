+++
title = "A cube united with a narrow cylinder wholly inside it"
tags = ["boolean", "union", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopfuse_simple/ZN4"

[volume]
value = 1000.0
exact = "1000: the union is the cube itself"
tex = '10^3'

[area]
value = 600.0
exact = "600: the cube's six faces"
tex = '6\cdot 10^2'
+++

A cube with x, y and z from 0 to 10.

A cylinder of radius 3/2, its axis the vertical line through (4, 6), from z = 3 to z = 7: strictly inside the cube, touching none of its faces. Make the union of the two, which is the cube itself.

**As a set:**

$$[0,10]^3 \cup \{\,(x-4)^2 + (y-6)^2 \le (3/2)^2,\ 3 \le z \le 7\,\}$$

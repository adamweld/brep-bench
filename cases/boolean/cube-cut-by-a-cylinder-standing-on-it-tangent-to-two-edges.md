+++
title = "A cube cut by a cylinder standing on its top, tangent to two of its edges"
tags = ["boolean", "cut", "analytic", "tangent", "touching"]
origin = "OCCT tests/boolean/bopcut_simple/ZO9"

[volume]
value = 1000.0
exact = "nothing is removed: 10³"
tex = '10^3'

[area]
value = 600.0
exact = "6·10²"
tex = '6\cdot 10^2'
+++

A cube of side 10 between the origin and (10, 10, 10), and a cylinder of radius 2.5 and height 4 standing on its top face, its axis the vertical line through (2.5, 2.5), from z = 10 to z = 14. The cylinder's base circle lies in the cube's top face and is tangent to its edges y = 0 and x = 0, at (2.5, 0) and (0, 2.5). The two bodies touch over the disc and nothing else. Cut the cylinder away from the cube: the two share no volume, and the result is the cube as it was.

**As a set:**

$$[0,10]^3 \setminus \{(x-2.5)^2 + (y-2.5)^2 \le 2.5^2,\ 10 \le z \le 14\}$$

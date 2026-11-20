+++
title = "A cube with a cylindrical void inside it"
tags = ["boolean", "cut", "analytic", "void"]
origin = "OCCT tests/boolean/bopcut_simple/ZN4"

[volume]
value = 949.7345175425633
exact = "10³ − π·2²·4"
tex = '10^3 - \pi\cdot 2^2\cdot 4'

[area]
value = 675.398223686155
exact = "600 + 2π·2·4 + 2π·2²"
tex = '600 + 2\pi\cdot 2\cdot 4 + 2\pi\cdot 2^2'
+++

A cube of side 10 between the origin and (10, 10, 10), with a cylinder cut out of its inside: radius 2, height 4, axis the vertical line through (5, 5), from z = 3 to z = 7. The cylinder is wholly inside the cube and touches none of its faces, so the result is a cube with a closed cylindrical void, a second boundary shell inside the first.

**As a set:**

$$[0,10]^3 \setminus \{(x-5)^2 + (y-5)^2 \le 2^2,\ 3 \le z \le 7\}$$

+++
title = "A cube united with a narrow cylinder through its centre, out of top and bottom"
tags = ["boolean", "union", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopfuse_simple/ZN3"

[volume]
value = 1075.398223686155
exact = "1000 + 4π·6: the cube and the two parts of the cylinder outside it, each of height 3 and area 4π"
tex = '1000 + 24\pi'

[area]
value = 675.398223686155
exact = "600 + 2·(2π·2·3): the cube's area (the discs the cylinder takes from its top and bottom are replaced by its own ends) plus the cylinder's wall outside the cube"
tex = '600 + 24\pi'
+++

A cube with x, y and z from 0 to 10.

A cylinder of radius 2, its axis the vertical line through (5, 5), from z = −3 to z = 13: it passes through the centre of the cube and comes out 3 below it and 3 above it. Make the union of the two.

**As a set:**

$$[0,10]^3 \cup \{\,(x-5)^2 + (y-5)^2 \le 2^2,\ -3 \le z \le 13\,\}$$

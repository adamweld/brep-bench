+++
title = "A cube united with a bar lying in one of its edges"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/A5"

[volume]
value = 1000.0
exact = "10·10·10"
tex = '10\cdot 10\cdot 10'

[area]
value = 600.0
exact = "6·10·10"
tex = '6\cdot 10\cdot 10'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a bar between the origin and (10, 3, 3): it runs the cube's whole length along x, hugging the cube's edge along the x axis, and four of its faces lie in the cube's faces (x = 0, x = 10, y = 0 and z = 0). Everything of the block lies inside the cube or on its surface, so the union is the cube itself.

**As a set:**

$$[0,10]^3 \cup \bigl([0,10]\times[0,3]^2\bigr)$$

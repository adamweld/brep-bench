+++
title = "A cube united with a slab through its middle"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/A9"

[volume]
value = 1000.0
exact = "10·10·10"
tex = '10\cdot 10\cdot 10'

[area]
value = 600.0
exact = "6·10·10"
tex = '6\cdot 10\cdot 10'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a slab between (0, 0, 3) and (10, 10, 6): it spans the cube both ways in x and y, so its four side faces lie in the cube's side faces, and it lies strictly between the cube's top and bottom. Everything of the block lies inside the cube or on its surface, so the union is the cube itself.

**As a set:**

$$[0,10]^3 \cup \bigl([0,10]^2\times[3,6]\bigr)$$

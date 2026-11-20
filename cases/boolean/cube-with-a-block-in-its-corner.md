+++
title = "A cube united with a block in one of its corners"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/B1"

[volume]
value = 1000.0
exact = "10·10·10"
tex = '10\cdot 10\cdot 10'

[area]
value = 600.0
exact = "6·10·10"
tex = '6\cdot 10\cdot 10'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a cube of side 4 between the origin and (4, 4, 4), which fills the corner at the origin: three of its faces lie in the cube's faces (x = 0, y = 0 and z = 0). Everything of the block lies inside the cube or on its surface, so the union is the cube itself.

**As a set:**

$$[0,10]^3 \cup [0,4]^3$$

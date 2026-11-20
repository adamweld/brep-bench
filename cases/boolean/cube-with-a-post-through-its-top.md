+++
title = "A cube with a post through its top"
tags = ["boolean", "union", "planar"]
origin = "OCCT boolean/bopfuse_simple/P3"

[volume]
value = 1045.0
exact = "10·10·10 + 3·3·5"
tex = '10\cdot 10\cdot 10 + 3\cdot 3\cdot 5'

[area]
value = 660.0
exact = "6·10·10 + 4·3·5"
tex = '6\cdot 10\cdot 10 + 4\cdot 3\cdot 5'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a post between (3, 3, 5) and (6, 6, 15): it starts inside the cube, leaves through the top face well away from its edges and stands 5 above it. No face of the post lies in a face of the cube.

**As a set:**

$$[0,10]^3 \cup \bigl([3,6]^2\times[5,15]\bigr)$$

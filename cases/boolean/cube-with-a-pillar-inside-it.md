+++
title = "A cube united with a pillar standing inside it"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/K1"

[volume]
value = 1000.0
exact = "10·10·10"
tex = '10\cdot 10\cdot 10'

[area]
value = 600.0
exact = "6·10·10"
tex = '6\cdot 10\cdot 10'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a square pillar between (3, 3, 0) and (6, 6, 10): strictly inside the cube across, and as tall as the cube, so its top lies in the cube's top face and its bottom in the cube's bottom face. Everything of the block lies inside the cube or on its surface, so the union is the cube itself.

**As a set:**

$$[0,10]^3 \cup \bigl([3,6]^2\times[0,10]\bigr)$$

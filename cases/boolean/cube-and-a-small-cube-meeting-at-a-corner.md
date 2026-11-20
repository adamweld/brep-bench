+++
title = "A cube and a smaller cube meeting at a corner, united"
tags = ["boolean", "union", "planar", "coincident", "non-manifold"]
origin = "OCCT boolean/bopfuse_simple/B4"

[volume]
value = 1064.0
exact = "10·10·10 + 4·4·4"
tex = '10\cdot 10\cdot 10 + 4\cdot 4\cdot 4'

[area]
value = 696.0
exact = "6·10·10 + 6·4·4"
tex = '6\cdot 10\cdot 10 + 6\cdot 4\cdot 4'
+++

A cube of side 10 between the origin and (10, 10, 10), and a cube of side 4 between (10, 10, 10) and (14, 14, 14), standing on the first's far corner. They share that one point and nothing else. Unite them. The result is the two cubes touching at a point, one body only in that sense: its volume and its area are those of the two cubes taken together.

**As a set:**

$$[0,10]^3 \cup [10,14]^3$$

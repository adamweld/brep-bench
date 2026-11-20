+++
title = "Two cubes meeting along an edge, united"
tags = ["boolean", "union", "planar", "coincident", "non-manifold"]
origin = "OCCT boolean/bopfuse_simple/A4"

[volume]
value = 2000.0
exact = "2·10·10·10"
tex = '2\cdot 10\cdot 10\cdot 10'

[area]
value = 1200.0
exact = "2·6·10·10"
tex = '2\cdot 6\cdot 10\cdot 10'
+++

Two cubes of side 10: the first between the origin and (10, 10, 10), the second between (10, 10, 0) and (20, 20, 10), standing diagonally beside it. They touch along the single edge from (10, 10, 0) to (10, 10, 10) and nowhere else. Unite them. The result is the two cubes touching along that line and nowhere else, one body only in that sense: its volume and its area are those of the two cubes taken together.

**As a set:**

$$[0,10]^3 \cup \bigl([10,20]^2\times[0,10]\bigr)$$

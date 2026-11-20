+++
title = "Two cubes meeting along part of an edge, united"
tags = ["boolean", "union", "planar", "coincident", "non-manifold"]
origin = "OCCT boolean/bopfuse_simple/B3"

[volume]
value = 2000.0
exact = "2·10·10·10"
tex = '2\cdot 10\cdot 10\cdot 10'

[area]
value = 1200.0
exact = "2·6·10·10"
tex = '2\cdot 6\cdot 10\cdot 10'
+++

Two cubes of side 10: the first between the origin and (10, 10, 10), the second between (10, 10, 5) and (20, 20, 15), standing diagonally beside it and raised by 5. Their vertical edges at x = y = 10 lie on one line and overlap in the segment from (10, 10, 5) to (10, 10, 10), which is all they share. Unite them. The result is the two cubes touching along that segment and nowhere else, one body only in that sense: its volume and its area are those of the two cubes taken together.

**As a set:**

$$[0,10]^3 \cup \bigl([10,20]^2\times[5,15]\bigr)$$

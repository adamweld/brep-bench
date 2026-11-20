+++
title = "A block with a small cube on one corner of its top"
tags = ["boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/E2, OCCT boolean/bfuse_simple/D9, OCCT boolean/bfuse_simple/E5"

[volume]
value = 1925
exact = "20·15·6 + 5³"
tex = '20\cdot 15\cdot 6 + 5^3'

[area]
value = 1120
exact = "2·(20·15 + 20·6 + 15·6) − 5² + (5² + 4·5²)"
tex = '2\,(20\cdot 15 + 20\cdot 6 + 15\cdot 6) - 5^2 + (5^2 + 4\cdot 5^2)'
+++

A block from (0, 0, 0) to (20, 15, 6) and a cube of side 5 from (0, 0, 6) to (5, 5, 11), both with plane faces. Unite them.

The cube stands on the block's top at the corner of it at the origin: its bottom is a 5 by 5 patch of the top, and its sides at x = 0 and y = 0 lie in the planes of the block's sides at x = 0 and y = 0. No volume overlaps. The union is the block with the cube on its corner, one solid, the block's two sides continuing up the cube's two sides as single flat faces.

**As a set:**

$$[0,20]\times[0,15]\times[0,6] \cup [0,5]^2\times[6,11]$$

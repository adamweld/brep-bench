+++
title = "A block turned three-four-five and raised through a cube's top, less the cube"
tags = ["boolean", "cut", "analytic", "planar", "turned"]
origin = "OCCT tests/boolean/boptuc_simple/P1"

[volume]
value = 390
exact = "18·5 for the part beside the cube, plus 60·5 for the part above it"
tex = '18\cdot 5 + 60\cdot 5'

[area]
value = 390
exact = "60 top, 18 bottom, 42 underside where the cube was, plus 32·5 round the upper block and 22·5 round the lower part"
tex = '60 + 18 + 42 + 32\cdot 5 + 22\cdot 5'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 10 by 6 by 10 turned about the upright, raised by 5. One corner of the block is (6, 1, 5), and the block's three edges from that corner run 10 along (4/5, 3/5, 0), 6 along (−3/5, 4/5, 0) and 10 straight up. Cut the cube away from the block.

The block runs from z = 5 to z = 15, leaving the cube through its top face z = 10. Its footprint is the rectangle with corners (6, 1), (14, 7), (10.4, 11.8) and (2.4, 5.8), area 60, overlapping the cube's footprint in a polygon of area 42; the cube's corner (10, 10) is strictly inside it.

What is left is the block's part above the cube, z from 10 to 15, over its whole footprint, and under it, from z = 5 to z = 10, a prism on the polygon with corners (10, 4), (14, 7), (10.4, 11.8), (8, 10), (10, 10), of area 18 and boundary 22, reaching out beyond the cube's side faces x = 10 and y = 10 as a ledge under the block.

**As a set:**

$$\{\,0 \le 4x + 3y - 27 \le 50,\ 0 \le -3x + 4y + 14 \le 30,\ 5 \le z \le 15\,\} \setminus [0,10]^3$$

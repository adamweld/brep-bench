+++
title = "A block turned three-four-five and half the cube's height, less the cube"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "turned"]
origin = "OCCT tests/boolean/boptuc_simple/L7"

[volume]
value = 90
exact = "5·18: the area of the footprint outside the cube, 5 high"
tex = '5\cdot 18'

[area]
value = 146
exact = "2·18 for the two ends, plus 5·22 for the walls round the footprint's boundary, which is 22 long"
tex = '2\cdot 18 + 5\cdot 22'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 10 by 6 by 5 turned about the upright. One corner of the block is (6, 1, 0), and the block's three edges from that corner run 10 along (4/5, 3/5, 0), 6 along (−3/5, 4/5, 0) and 5 straight up. Cut the cube away from the block.

The block stands on the cube's base plane z = 0 and its top is halfway up the cube. Its footprint is the rectangle with corners (6, 1), (14, 7), (10.4, 11.8) and (2.4, 5.8); two corners are inside the cube's footprint and two outside it, and the cube's corner (10, 10) is strictly inside it.

What is left is a prism 5 high on the polygon with corners (10, 4), (14, 7), (10.4, 11.8), (8, 10), (10, 10), of area 18 and boundary 22, standing on z = 0.

**As a set:**

$$\{\,0 \le 4x + 3y - 27 \le 50,\ 0 \le -3x + 4y + 14 \le 30,\ 0 \le z \le 5\,\} \setminus [0,10]^3$$

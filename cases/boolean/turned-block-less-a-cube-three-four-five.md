+++
title = "A block turned three-four-five across a cube's corner, less the cube"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "turned"]
origin = "OCCT tests/boolean/boptuc_simple/C6"

[volume]
value = 180
exact = "10·18: the area of the footprint outside the cube, 10 high"
tex = '10\cdot 18'

[area]
value = 256
exact = "2·18 for the two ends, plus 10·22 for the walls round the footprint's boundary, which is 22 long"
tex = '2\cdot 18 + 10\cdot 22'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 10 by 6 by 10 turned about the upright. One corner of the block is (6, 1, 0), and the block's three edges from that corner run 10 along (4/5, 3/5, 0), 6 along (−3/5, 4/5, 0) and 10 straight up. Cut the cube away from the block.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. The block's corners (6, 1) and (2.4, 5.8) are inside the cube's footprint; (14, 7) and (10.4, 11.8) are outside it, and the cube's corner (10, 10) is strictly inside the block's footprint. The block's side from (6, 1) crosses the cube's face x = 10 at (10, 4); its side from (2.4, 5.8) crosses the cube's face y = 10 at (8, 10).

What is left is a prism 10 high on the polygon with corners (10, 4), (14, 7), (10.4, 11.8), (8, 10), (10, 10), of area 18 and boundary 22, standing on z = 0.

**As a set:**

$$\{\,0 \le 4x + 3y - 27 \le 50,\ 0 \le -3x + 4y + 14 \le 30,\ 0 \le z \le 10\,\} \setminus [0,10]^3$$

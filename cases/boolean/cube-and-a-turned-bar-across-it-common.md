+++
title = "A cube and a long thin block turned across it, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident", "vertex-on-plane"]
origin = "OCCT tests/boolean/bopcommon_simple/C3"

[volume]
value = 125
exact = "a prism 10 high on the parallelogram with corners (0, 1), (10, 17/2), (10, 39/4), (0, 9/4), which has base 10 and height 5/4: 10·10·5/4"
tex = '10\cdot 10\cdot\tfrac54'

[area]
value = 300
exact = "two parallelograms of 25/2, two sides 10 high on the 5/4 sides, two on the 25/2 sides: 25 + 10·(5/4 + 25/2 + 5/4 + 25/2)"
tex = '2\cdot\tfrac{25}{2} + 10\left(2\cdot\tfrac54 + 2\cdot\tfrac{25}{2}\right)'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 20 by 1 by 10 turned about the upright. One corner of the block is (0, 1, 0), and the block's three edges from that corner run 20 along (4/5, 3/5, 0), 1 along (−3/5, 4/5, 0) and 10 straight up. Make the solid common to both.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. The block's corner (0, 1, 0) lies on the cube's face x = 0; its other three corners, (16, 13), (77/5, 69/5) and (−3/5, 9/5) on the xy plane, are outside the cube, and none of the cube's corners is inside the block. The block crosses the cube from its face x = 0 to its face x = 10, as a band.

The common part is a prism 10 high on the parallelogram with corners (0, 1), (10, 17/2), (10, 39/4) and (0, 9/4): its long sides lie in the block's two long sides and its short sides in the cube's faces x = 0 and x = 10.

**As a set:**

$$[0,10]^3 \cap \{\,0 \le 4x + 3y - 3 \le 100,\ 0 \le -3x + 4y - 4 \le 5,\ 0 \le z \le 10\,\}$$

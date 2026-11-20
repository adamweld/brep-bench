+++
title = "A cube less a turned block that covers a cube corner, two of its corners in the cube"
tags = ["boolean", "cut", "analytic", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/J8"

[volume]
value = 870.4166666666666
exact = "10·(100 − 311/24) = 10·2089/24: the cube's square less the 311/24 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{311}{24}\right) = 10\cdot \tfrac{2089}{24}'

[area]
value = 584.0833333333334
exact = "2·2089/24 for top and bottom, plus 10·41 for the walls round the footprint's boundary, which is 41 long"
tex = '2\cdot \tfrac{2089}{24} + 10\cdot 41'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 10 turned about the upright. One corner of the block is (0, -2, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (0, -2), (4, 1), (1, 5), (-3, 2): of them 2 inside the cube's footprint, 2 outside it.

What is left is a prism 10 high on the polygon with corners (0, 10), (10, 10), (10, 0), (8/3, 0), (4, 1), (1, 5), (0, 17/4), standing on z = 0.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y + 6 \le 25,\ 0 \le -3x + 4y + 8 \le 25,\ 0 \le z \le 10\,\}$$

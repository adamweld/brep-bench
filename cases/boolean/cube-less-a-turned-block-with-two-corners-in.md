+++
title = "A cube less a turned block with two corners in the cube"
tags = ["boolean", "cut", "analytic", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/L3"

[volume]
value = 802.0833333333334
exact = "10·(100 − 475/24) = 10·1925/24: the cube's square less the 475/24 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{475}{24}\right) = 10\cdot \tfrac{1925}{24}'

[area]
value = 610.4166666666666
exact = "2·1925/24 for top and bottom, plus 10·45 for the walls round the footprint's boundary, which is 45 long"
tex = '2\cdot \tfrac{1925}{24} + 10\cdot 45'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 10 turned about the upright. One corner of the block is (1, -1, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (1, -1), (5, 2), (2, 6), (-2, 3): of them 2 inside the cube's footprint, 2 outside it.

What is left is two separate solids, each a prism 10 high standing on z = 0: one on the polygon with corners (0, 10), (10, 10), (10, 0), (7/3, 0), (5, 2), (2, 6), (0, 9/2), one on the polygon with corners (0, 0), (0, 1/3), (1/4, 0).

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y - 1 \le 25,\ 0 \le -3x + 4y + 7 \le 25,\ 0 \le z \le 10\,\}$$

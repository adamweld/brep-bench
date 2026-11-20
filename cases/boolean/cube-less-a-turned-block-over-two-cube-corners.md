+++
title = "A cube less a turned block that covers two cube corners"
tags = ["boolean", "cut", "analytic", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/K6"

[volume]
value = 210.41666666666666
exact = "10·(100 − 1895/24) = 10·505/24: the cube's square less the 1895/24 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{1895}{24}\right) = 10\cdot \tfrac{505}{24}'

[area]
value = 332.0833333333333
exact = "2·505/24 for top and bottom, plus 10·29 for the walls round the footprint's boundary, which is 29 long"
tex = '2\cdot \tfrac{505}{24} + 10\cdot 29'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 25/2 by 10 by 10 turned about the upright. One corner of the block is (5, -4, 0), and the block's three edges from that corner run 25/2 along (4/5, 3/5, 0), 10 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (5, -4), (15, 7/2), (9, 23/2), (-1, 4): of them 4 outside it. 2 of the cube's corners lie strictly inside the block's footprint.

What is left is two separate solids, each a prism 10 high standing on z = 0: one on the polygon with corners (0, 10), (7, 10), (0, 19/4), one on the polygon with corners (0, 0), (0, 8/3), (2, 0).

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y - 8 \le \tfrac{125}{2},\ 0 \le -3x + 4y + 31 \le 50,\ 0 \le z \le 10\,\}$$

+++
title = "A cube less a turned block inside it with three corners on its sides"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/G2"

[volume]
value = 666.6666666666666
exact = "10·(100 − 100/3) = 10·200/3: the cube's square less the 100/3 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{100}{3}\right) = 10\cdot \tfrac{200}{3}'

[area]
value = 800.0
exact = "2·200/3 for top and bottom, plus 10·200/3 for the walls round the footprint's boundary, which is 200/3 long"
tex = '2\cdot \tfrac{200}{3} + 10\cdot \tfrac{200}{3}'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 10 by 10/3 by 10 turned about the upright. One corner of the block is (2, 0, 0), and the block's three edges from that corner run 10 along (4/5, 3/5, 0), 10/3 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (2, 0), (10, 6), (8, 26/3), (0, 8/3): of them 1 inside the cube's footprint, 3 on its sides.

What is left is three separate solids, each a prism 10 high standing on z = 0: one on the polygon with corners (10, 0), (2, 0), (10, 6), one on the polygon with corners (0, 10), (10, 10), (10, 6), (8, 26/3), (0, 8/3), one on the polygon with corners (0, 0), (0, 8/3), (2, 0). The block's corners (2, 0), (10, 6), (0, 8/3) lie on the cube's side with the block inside the cube there, so at each the material left is only a line thick across the block's upright edge: the pieces or walls on either side of it meet along that edge and no more.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y - 8 \le 50,\ 0 \le -3x + 4y + 6 \le \tfrac{50}{3},\ 0 \le z \le 10\,\}$$

+++
title = "A cube less a turned block that covers two cube corners and has a corner on its side"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/I6"

[volume]
value = 277.6041666666667
exact = "10·(100 − 6935/96) = 10·2665/96: the cube's square less the 6935/96 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{6935}{96}\right) = 10\cdot \tfrac{2665}{96}'

[area]
value = 410.5208333333333
exact = "2·2665/96 for top and bottom, plus 10·71/2 for the walls round the footprint's boundary, which is 71/2 long"
tex = '2\cdot \tfrac{2665}{96} + 10\cdot \tfrac{71}{2}'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 25/2 by 10 by 10 turned about the upright. One corner of the block is (0, -2, 0), and the block's three edges from that corner run 25/2 along (4/5, 3/5, 0), 10 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (0, -2), (10, 11/2), (4, 27/2), (-6, 6): of them 1 on its boundary, 3 outside it.

What is left is two separate solids, each a prism 10 high standing on z = 0: one on the polygon with corners (10, 0), (8/3, 0), (10, 11/2), one on the polygon with corners (10, 10), (10, 11/2), (53/8, 10). The block's corner (10, 11/2) lies on the cube's side with the block inside the cube there, so at it the material left is only a line thick across the block's upright edge: the pieces or walls on either side of it meet along that edge and no more.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y + 6 \le \tfrac{125}{2},\ 0 \le -3x + 4y + 8 \le 50,\ 0 \le z \le 10\,\}$$

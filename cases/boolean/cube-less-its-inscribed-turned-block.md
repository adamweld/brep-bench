+++
title = "A cube less the turned square block inscribed in its footprint"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/F5"

[volume]
value = 489.7959183673469
exact = "10·(100 − 2500/49) = 10·2400/49: the cube's square less the 2500/49 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{2500}{49}\right) = 10\cdot \tfrac{2400}{49}'

[area]
value = 783.6734693877551
exact = "2·2400/49 for top and bottom, plus 10·480/7 for the walls round the footprint's boundary, which is 480/7 long"
tex = '2\cdot \tfrac{2400}{49} + 10\cdot \tfrac{480}{7}'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 50/7 by 50/7 by 10 turned about the upright. One corner of the block is (30/7, 0, 0), and the block's three edges from that corner run 50/7 along (4/5, 3/5, 0), 50/7 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (30/7, 0), (10, 30/7), (40/7, 10), (0, 40/7): of them 4 on its sides.

What is left is four separate solids, each a prism 10 high standing on z = 0: one on the polygon with corners (10, 0), (30/7, 0), (10, 30/7), one on the polygon with corners (10, 10), (10, 30/7), (40/7, 10), one on the polygon with corners (0, 10), (40/7, 10), (0, 40/7), one on the polygon with corners (0, 0), (0, 40/7), (30/7, 0). The block's corners (30/7, 0), (10, 30/7), (40/7, 10), (0, 40/7) lie on the cube's side with the block inside the cube there, so at each the material left is only a line thick across the block's upright edge: the pieces or walls on either side of it meet along that edge and no more.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y - \tfrac{120}{7} \le \tfrac{250}{7},\ 0 \le -3x + 4y + \tfrac{90}{7} \le \tfrac{250}{7},\ 0 \le z \le 10\,\}$$

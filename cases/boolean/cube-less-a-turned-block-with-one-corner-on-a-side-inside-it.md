+++
title = "A cube less a turned block inside it with one corner on a side"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/H9"

[volume]
value = 750.0
exact = "10·(100 − 25) = 10·75: the cube's square less the 25 of it the block's footprint covers"
tex = '10\left(100 - 25\right) = 10\cdot 75'

[area]
value = 750.0
exact = "2·75 for top and bottom, plus 10·60 for the walls round the footprint's boundary, which is 60 long"
tex = '2\cdot 75 + 10\cdot 60'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 10 turned about the upright. One corner of the block is (3, 1, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (3, 1), (7, 4), (4, 8), (0, 5): of them 3 inside the cube's footprint, 1 on its boundary.

What is left is a prism with a through hole, 10 high on the polygon with corners (0, 10), (10, 10), (10, 0), (0, 0), (0, 5) and a hole with corners (3, 1), (7, 4), (4, 8), (0, 5), standing on z = 0. The block's corner (0, 5) lies on the cube's side with the block inside the cube there, so at it the material left is only a line thick across the block's upright edge: the pieces or walls on either side of it meet along that edge and no more.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y - 15 \le 25,\ 0 \le -3x + 4y + 5 \le 25,\ 0 \le z \le 10\,\}$$

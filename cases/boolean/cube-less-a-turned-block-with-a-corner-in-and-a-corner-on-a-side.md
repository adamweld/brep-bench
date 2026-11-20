+++
title = "A cube less a turned block with one corner in the cube and one on its side"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/C4"

[volume]
value = 906.25
exact = "10·(100 − 75/8) = 10·725/8: the cube's square less the 75/8 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{75}{8}\right) = 10\cdot \tfrac{725}{8}'

[area]
value = 606.25
exact = "2·725/8 for top and bottom, plus 10·85/2 for the walls round the footprint's boundary, which is 85/2 long"
tex = '2\cdot \tfrac{725}{8} + 10\cdot \tfrac{85}{2}'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 10 turned about the upright. One corner of the block is (-1, 0, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (-1, 0), (3, 3), (0, 7), (-4, 4): of them 1 inside the cube's footprint, 1 on its boundary, 2 outside it.

What is left is a prism 10 high on the polygon with corners (0, 10), (10, 10), (10, 0), (0, 0), (0, 3/4), (3, 3), (0, 7), standing on z = 0.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y + 4 \le 25,\ 0 \le -3x + 4y - 3 \le 25,\ 0 \le z \le 10\,\}$$

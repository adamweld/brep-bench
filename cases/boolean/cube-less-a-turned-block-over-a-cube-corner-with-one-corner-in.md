+++
title = "A cube less a turned block that covers a cube corner, one corner in the cube and one on its side"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/I5"

[volume]
value = 916.6666666666666
exact = "10·(100 − 25/3) = 10·275/3: the cube's square less the 25/3 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{25}{3}\right) = 10\cdot \tfrac{275}{3}'

[area]
value = 583.3333333333334
exact = "2·275/3 for top and bottom, plus 10·40 for the walls round the footprint's boundary, which is 40 long"
tex = '2\cdot \tfrac{275}{3} + 10\cdot 40'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 10 turned about the upright. One corner of the block is (-1, -2, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (-1, -2), (3, 1), (0, 5), (-4, 2): of them 1 inside the cube's footprint, 1 on its boundary, 2 outside it.

What is left is a prism 10 high on the polygon with corners (0, 10), (10, 10), (10, 0), (5/3, 0), (3, 1), (0, 5), standing on z = 0.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y + 10 \le 25,\ 0 \le -3x + 4y + 5 \le 25,\ 0 \le z \le 10\,\}$$

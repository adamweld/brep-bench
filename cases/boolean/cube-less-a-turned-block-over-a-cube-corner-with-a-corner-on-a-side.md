+++
title = "A cube less a turned block that covers a cube corner and has a corner on its side"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/D4"

[volume]
value = 993.3333333333334
exact = "10·(100 − 2/3) = 10·298/3: the cube's square less the 2/3 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{2}{3}\right) = 10\cdot \tfrac{298}{3}'

[area]
value = 592.0
exact = "2·298/3 for top and bottom, plus 10·118/3 for the walls round the footprint's boundary, which is 118/3 long"
tex = '2\cdot \tfrac{298}{3} + 10\cdot \tfrac{118}{3}'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 10 turned about the upright. One corner of the block is (-3, -3, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (-3, -3), (1, 0), (-2, 4), (-6, 1): of them 1 on its boundary, 3 outside it. One of the cube's corners lies strictly inside the block's footprint.

What is left is a prism 10 high on the polygon with corners (0, 10), (10, 10), (10, 0), (1, 0), (0, 4/3), standing on z = 0.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y + 21 \le 25,\ 0 \le -3x + 4y + 3 \le 25,\ 0 \le z \le 10\,\}$$

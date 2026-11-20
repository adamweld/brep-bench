+++
title = "A cube less a turned block that only touches the cube's corner"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/D7"

[volume]
value = 1000.0
exact = "10·(100 − 0) = 10·100: the cube's square less the 0 of it the block's footprint covers"
tex = '10\left(100 - 0\right) = 10\cdot 100'

[area]
value = 600.0
exact = "2·100 for top and bottom, plus 10·40 for the walls round the footprint's boundary, which is 40 long"
tex = '2\cdot 100 + 10\cdot 40'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 10 turned about the upright. One corner of the block is (-4, -3, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (-4, -3), (0, 0), (-3, 4), (-7, 1): of them 1 on its boundary, 3 outside it. One of the cube's corners lies strictly inside the block's footprint.

What is left is a prism 10 high on the polygon with corners (0, 10), (10, 10), (10, 0), (0, 0), standing on z = 0.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y + 25 \le 25,\ 0 \le -3x + 4y - 0 \le 25,\ 0 \le z \le 10\,\}$$

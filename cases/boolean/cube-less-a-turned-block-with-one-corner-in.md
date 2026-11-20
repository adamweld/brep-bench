+++
title = "A cube less a turned block with one corner in the cube"
tags = ["boolean", "cut", "analytic", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/K4"

[volume]
value = 989.5833333333334
exact = "10·(100 − 25/24) = 10·2375/24: the cube's square less the 25/24 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{25}{24}\right) = 10\cdot \tfrac{2375}{24}'

[area]
value = 606.25
exact = "2·2375/24 for top and bottom, plus 10·245/6 for the walls round the footprint's boundary, which is 245/6 long"
tex = '2\cdot \tfrac{2375}{24} + 10\cdot \tfrac{245}{6}'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 10 turned about the upright. One corner of the block is (-3, -2, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (-3, -2), (1, 1), (-2, 5), (-6, 2): of them 1 inside the cube's footprint, 3 outside it.

What is left is a prism 10 high on the polygon with corners (0, 10), (10, 10), (10, 0), (0, 0), (0, 1/4), (1, 1), (0, 7/3), standing on z = 0.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y + 18 \le 25,\ 0 \le -3x + 4y - 1 \le 25,\ 0 \le z \le 10\,\}$$

+++
title = "A cube less a turned block with three corners in the cube"
tags = ["boolean", "cut", "analytic", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/K2"

[volume]
value = 791.6666666666666
exact = "10·(100 − 125/6) = 10·475/6: the cube's square less the 125/6 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{125}{6}\right) = 10\cdot \tfrac{475}{6}'

[area]
value = 658.3333333333334
exact = "2·475/6 for top and bottom, plus 10·50 for the walls round the footprint's boundary, which is 50 long"
tex = '2\cdot \tfrac{475}{6} + 10\cdot 50'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 10 turned about the upright. One corner of the block is (1, 1, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (1, 1), (5, 4), (2, 8), (-2, 5): of them 3 inside the cube's footprint, 1 outside it.

What is left is a prism 10 high on the polygon with corners (0, 10), (10, 10), (10, 0), (0, 0), (0, 7/3), (1, 1), (5, 4), (2, 8), (0, 13/2), standing on z = 0.

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y - 7 \le 25,\ 0 \le -3x + 4y - 1 \le 25,\ 0 \le z \le 10\,\}$$

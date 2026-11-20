+++
title = "A cube less a turned block lying across it, cutting off four corners"
tags = ["boolean", "cut", "analytic", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/K7"

[volume]
value = 208.33333333333334
exact = "10·(100 − 475/6) = 10·125/6: the cube's square less the 475/6 of it the block's footprint covers"
tex = '10\left(100 - \tfrac{475}{6}\right) = 10\cdot \tfrac{125}{6}'

[area]
value = 441.6666666666667
exact = "2·125/6 for top and bottom, plus 10·40 for the walls round the footprint's boundary, which is 40 long"
tex = '2\cdot \tfrac{125}{6} + 10\cdot 40'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 10 by 10 by 10 turned about the upright. One corner of the block is (3, -3, 0), and the block's three edges from that corner run 10 along (4/5, 3/5, 0), 10 along (−3/5, 4/5, 0) and 10 straight up. Cut the block away from the cube.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (3, -3), (11, 3), (5, 11), (-3, 5): of them 4 outside it.

What is left is four separate solids, each a prism 10 high standing on z = 0: one on the polygon with corners (10, 0), (7, 0), (10, 9/4), one on the polygon with corners (10, 10), (10, 13/3), (23/4, 10), one on the polygon with corners (0, 10), (11/3, 10), (0, 29/4), one on the polygon with corners (0, 0), (0, 1), (3/4, 0).

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y - 3 \le 50,\ 0 \le -3x + 4y + 21 \le 50,\ 0 \le z \le 10\,\}$$

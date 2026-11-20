+++
title = "A cube cut by a turned bar across its corner region, flush with its base"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "vertex-on-plane"]
origin = "OCCT tests/boolean/bopcut_simple/L4"

[volume]
value = 838.5416666666666
exact = "1000 − d·A, d = 5, A = 775/24 the area where the footprints overlap"
tex = '1000 - d\,A,\quad d = 5,\ A = 775/24'

[area]
value = 591.6666666666666
exact = "600 + d(Lbc − Lcb): d = 5, Lbc = 155/12 the block's footprint boundary inside the cube, Lcb = 175/12 the cube's footprint boundary inside the block; the floor replaces the cube's face over the overlap, walls add d·Lbc, side faces lose d·Lcb"
tex = '600 + d\,(L_{bc} - L_{cb})'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block turned about the vertical, standing from z = 0 up to z = 5. The block's footprint is a rectangle 15 by 5, turned about the vertical: from its corner (0, 3) its edges run 15 along (4/5, 3/5) and 5 along (−3/5, 4/5). Its other footprint corners are (12, 12), (9, 16) and (-3, 7). Cut the block away from the cube.

The footprints overlap in the quadrilateral (0, 3), (28/3, 10), (1, 10), (0, 37/4), area 775/24: neither footprint has a corner of the other strictly inside it. The block's corner (0, 3) is on the cube's face x = 0, and the block's two long sides cross the cube's footprint and leave it through the side y = 10, the one at (28/3, 10) and the other at (1, 10), and through x = 0 at (0, 37/4). The block's base lies in the cube's base plane z = 0 and its top is halfway up the cube, at z = 5, so the cut hollows the cube's foot to a height of 5 beneath that polygon: a pocket open on the cube's base and on the side faces of the cube that the block runs out through.

**As a set:**

$$[0,10]^3 \setminus \{\, 0 \le 4x + 3y - 9 \le 75,\ 0 \le -3x + 4y - 12 \le 25,\ 0 \le z \le 5 \,\}$$

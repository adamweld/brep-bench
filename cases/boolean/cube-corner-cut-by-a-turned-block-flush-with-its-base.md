+++
title = "A cube's corner cut away by a turned block, flush with its base"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "vertex-on-plane"]
origin = "OCCT tests/boolean/bopcut_simple/M5"

[volume]
value = 983.125
exact = "1000 − d·A, d = 5, A = 27/8 the area where the footprints overlap"
tex = '1000 - d\,A,\quad d = 5,\ A = 27/8'

[area]
value = 592.5
exact = "600 + d(Lbc − Lcb): d = 5, Lbc = 15/4 the block's footprint boundary inside the cube, Lcb = 21/4 the cube's footprint boundary inside the block; the floor replaces the cube's face over the overlap, walls add d·Lbc, side faces lose d·Lcb"
tex = '600 + d\,(L_{bc} - L_{cb})'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block turned about the vertical, standing from z = 0 up to z = 5. The block's footprint is a rectangle 5 by 5, turned about the vertical: from its corner (10, 7) its edges run 5 along (4/5, 3/5) and 5 along (−3/5, 4/5). Its other footprint corners are (14, 10), (11, 14) and (7, 11). Cut the block away from the cube.

The block's corner (10, 7) is on the cube's face x = 10, and the cube's corner (10, 10) is strictly inside the block's footprint; none of the block's corners is inside the cube's. The footprints overlap in the triangle (10, 7), (10, 10), (31/4, 10), area 27/8. The block's base lies in the cube's base plane z = 0 and its top is halfway up the cube, at z = 5, so the cut hollows the cube's foot to a height of 5 beneath that polygon: a pocket open on the cube's base and on the side faces of the cube that the block runs out through.

**As a set:**

$$[0,10]^3 \setminus \{\, 0 \le 4x + 3y - 61 \le 25,\ 0 \le -3x + 4y + 2 \le 25,\ 0 \le z \le 5 \,\}$$

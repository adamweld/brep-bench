+++
title = "A cube notched from above by a turned block across two of its sides"
tags = ["boolean", "cut", "analytic", "planar", "general-position"]
origin = "OCCT tests/boolean/bopcut_simple/P2"

[volume]
value = 915.8333333333334
exact = "1000 − d·A, d = 5, A = 101/6 the area where the footprints overlap"
tex = '1000 - d\,A,\quad d = 5,\ A = 101/6'

[area]
value = 610.0
exact = "600 + d(Lbc − Lcb): d = 5, Lbc = 55/6 the block's footprint boundary inside the cube, Lcb = 43/6 the cube's footprint boundary inside the block; the floor replaces the cube's face over the overlap, walls add d·Lbc, side faces lose d·Lcb"
tex = '600 + d\,(L_{bc} - L_{cb})'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block turned about the vertical, standing from z = 5 up to z = 15. The block's footprint is a rectangle 5 by 5, turned about the vertical: from its corner (8, 5) its edges run 5 along (4/5, 3/5) and 5 along (−3/5, 4/5). Its other footprint corners are (12, 8), (9, 12) and (5, 9). Cut the block away from the cube.

Two of the block's corners, (8, 5) and (5, 9), are strictly inside the cube's footprint, and the cube's corner (10, 10) is strictly inside the block's; no corner of either lies on a face of the other. The footprints overlap in the pentagon (8, 5), (10, 13/2), (10, 10), (19/3, 10), (5, 9), area 101/6. The block's lower half (z from 5 to 10) lies inside the cube's upper half and its upper half stands clear above the cube's top face z = 10, so the cut takes a notch 5 deep out of the cube's top beneath that polygon, open on the top and on the side faces of the cube that the block runs out through.

**As a set:**

$$[0,10]^3 \setminus \{\, 0 \le 4x + 3y - 47 \le 25,\ 0 \le -3x + 4y + 4 \le 25,\ 5 \le z \le 15 \,\}$$

+++
title = "A cube notched from above by a long turned bar, neither with a corner in the other"
tags = ["boolean", "cut", "analytic", "planar", "general-position"]
origin = "OCCT tests/boolean/bopcut_simple/Q1"

[volume]
value = 859.375
exact = "1000 − d·A, d = 5, A = 225/8 the area where the footprints overlap"
tex = '1000 - d\,A,\quad d = 5,\ A = 225/8'

[area]
value = 583.3333333333334
exact = "600 + d(Lbc − Lcb): d = 5, Lbc = 45/4 the block's footprint boundary inside the cube, Lcb = 175/12 the cube's footprint boundary inside the block; the floor replaces the cube's face over the overlap, walls add d·Lbc, side faces lose d·Lcb"
tex = '600 + d\,(L_{bc} - L_{cb})'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block turned about the vertical, standing from z = 5 up to z = 15. The block's footprint is a rectangle 20 by 5, turned about the vertical: from its corner (-6, -1) its edges run 20 along (4/5, 3/5) and 5 along (−3/5, 4/5). Its other footprint corners are (10, 11), (7, 15) and (-9, 3). Cut the block away from the cube.

No corner of the block's footprint is inside the cube's and no corner of the cube's is inside the block's: the bar crosses the cube's footprint from side to side, its long sides cutting two sides of the cube each. The footprints overlap in the polygon (0, 7/2), (26/3, 10), (1/3, 10), (0, 39/4), area 225/8. The block's lower half (z from 5 to 10) lies inside the cube's upper half and its upper half stands clear above the cube's top face z = 10, so the cut takes a notch 5 deep out of the cube's top beneath that polygon, open on the top and on the side faces of the cube that the block runs out through.

**As a set:**

$$[0,10]^3 \setminus \{\, 0 \le 4x + 3y + 27 \le 100,\ 0 \le -3x + 4y - 14 \le 25,\ 5 \le z \le 15 \,\}$$

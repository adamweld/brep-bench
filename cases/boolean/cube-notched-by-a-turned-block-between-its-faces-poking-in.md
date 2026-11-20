+++
title = "A cube notched in its side by a turned block that lies between its top and base"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "vertex-on-plane"]
origin = "OCCT tests/boolean/bopcut_simple/Q7"

[volume]
value = 921.875
exact = "1000 − d·A, d = 5, A = 125/8 the area where the footprints overlap"
tex = '1000 - d\,A,\quad d = 5,\ A = 125/8'

[area]
value = 656.25
exact = "600 + 2A + d(Lbc − Lcb): A = 125/8 the overlap, d = 5, Lbc = 45/4 the block's footprint boundary inside the cube, Lcb = 25/4 the cube's footprint boundary inside the block; floor and ceiling A each, walls d·Lbc, side faces lose d·Lcb"
tex = '600 + 2A + d\,(L_{bc} - L_{cb})'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block turned about the vertical, standing from z = 5/2 up to z = 15/2. The block's footprint is a rectangle 5 by 5, turned about the vertical: from its corner (5, 6) its edges run 5 along (4/5, 3/5) and 5 along (−3/5, 4/5). Its other footprint corners are (9, 9), (6, 13) and (2, 10). Cut the block away from the cube.

Two of the block's corners, (5, 6) and (9, 9), are strictly inside the cube's footprint, a third, (2, 10), is on the cube's face y = 10, and the fourth is outside. The footprints overlap in the quadrilateral (5, 6), (9, 9), (33/4, 10), (2, 10), area 125/8. The block lies between the planes z = 2.5 and z = 7.5, wholly between the cube's base and top, so the cut takes a notch 5 high out of the cube's side over that polygon, closed above and below by the cube.

**As a set:**

$$[0,10]^3 \setminus \{\, 0 \le 4x + 3y - 38 \le 25,\ 0 \le -3x + 4y - 9 \le 25,\ 5/2 \le z \le 15/2 \,\}$$

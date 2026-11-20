+++
title = "A cube's top corner notched from above by a turned block"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "vertex-on-plane"]
origin = "OCCT tests/boolean/bopcut_simple/P1"

[volume]
value = 970.0
exact = "1000 − d·A, d = 5, A = 6 the area where the footprints overlap"
tex = '1000 - d\,A,\quad d = 5,\ A = 6'

[area]
value = 590.0
exact = "600 + d(Lbc − Lcb): d = 5, Lbc = 5 the block's footprint boundary inside the cube, Lcb = 7 the cube's footprint boundary inside the block; the floor replaces the cube's face over the overlap, walls add d·Lbc, side faces lose d·Lcb"
tex = '600 + d\,(L_{bc} - L_{cb})'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block turned about the vertical, standing from z = 5 up to z = 15. The block's footprint is a rectangle 5 by 5, turned about the vertical: from its corner (0, 7) its edges run 5 along (4/5, 3/5) and 5 along (−3/5, 4/5). Its other footprint corners are (4, 10), (1, 14) and (-3, 11). Cut the block away from the cube.

Two of the block's corners, (0, 7) and (4, 10), are on the cube's faces x = 0 and y = 10, and the cube's corner (0, 10) is strictly inside the block's footprint; the block's other corners are outside. The footprints overlap in the triangle (0, 7), (4, 10), (0, 10), area 6. The block's lower half (z from 5 to 10) lies inside the cube's upper half and its upper half stands clear above the cube's top face z = 10, so the cut takes a notch 5 deep out of the cube's top beneath that polygon, open on the top and on the side faces of the cube that the block runs out through.

**As a set:**

$$[0,10]^3 \setminus \{\, 0 \le 4x + 3y - 21 \le 25,\ 0 \le -3x + 4y - 28 \le 25,\ 5 \le z \le 15 \,\}$$

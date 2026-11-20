+++
title = "A cube's top edge notched by a block flush with its side"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/P4"

[volume]
value = 875.0
exact = "1000 − 5·5·5: the notch is the box [0, 5] × [2, 7] × [5, 10]"
tex = '1000 - 5\cdot5\cdot5'

[area]
value = 650.0
exact = "600 − 25 (side x = 0) − 25 (top) + 25 (floor) + 25 (wall x = 5) + 2·25 (walls y = 2, 7)"
tex = '600 - 25 - 25 + 25 + 25 + 2\cdot 25'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (0, 2, 5) to (5, 7, 15): its footprint lies within the cube's, one of its faces, x = 0, is in the cube's face x = 0, and it stands from the cube's mid-height up to 5 above its top face. Cut the block away from the cube. The cube loses the box from (0, 2, 5) to (5, 7, 10), a notch in the middle of its top edge along x = 0, open on the top and on the side x = 0.

**As a set:**

$$[0,10]^3 \setminus \bigl([0,5]\times[2,7]\times[5,15]\bigr)$$

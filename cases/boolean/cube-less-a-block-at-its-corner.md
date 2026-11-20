+++
title = "A cube less a block that touches it at one point"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "touching"]
origin = "OCCT tests/boolean/bopcut_simple/B4"

[volume]
value = 1000
exact = "the cube is untouched: 10³"
tex = '10^3'

[area]
value = 600
exact = "6·10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (10, 10, −10) to (20, 20, 0). The block lies below the plane z = 0 of the cube's base and beside the cube, and the two meet in one point only, the cube's corner (10, 10, 0). Cut the block away from the cube.

Nothing of the cube lies inside the block, so what is left is the cube.

**As a set:**

$$[0,10]^3 \setminus \bigl([10,20]\times[10,20]\times[-10,0]\bigr)$$

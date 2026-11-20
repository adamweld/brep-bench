+++
title = "A cube less a block against its face"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "touching"]
origin = "OCCT tests/boolean/bopcut_simple/A3"

[volume]
value = 1000
exact = "the cube is untouched: 10³"
tex = '10^3'

[area]
value = 600
exact = "6·10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (10, 5, 0) to (20, 15, 10). The block stands beside the cube on the xy plane, as tall as the cube, and touches the cube only along part of its face x = 10: the block's face x = 10 covers the cube's face from y = 5 to y = 10 and z = 0 to z = 10. Cut the block away from the cube.

Nothing of the cube lies inside the block, so what is left is the cube.

**As a set:**

$$[0,10]^3 \setminus \bigl([10,20]\times[5,15]\times[0,10]\bigr)$$

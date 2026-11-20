+++
title = "A cube cut through by a square prism as tall as itself"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/A9"

[volume]
value = 840
exact = "1000 − 4·4·10"
tex = '10^3 - 4\cdot 4\cdot 10'

[area]
value = 728
exact = "6·10² − 2·4² + 4·4·10: the top and bottom lose the opening, the hole adds four walls"
tex = '6\cdot 10^2 - 2\cdot 4^2 + 4\cdot 4\cdot 10'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (3, 3, 0) to (7, 7, 10): 4 by 4 and exactly as tall as the cube, standing on the xy plane inside the cube's footprint, so that its top and its bottom lie in the planes of the cube's top and bottom. Cut the block away from the cube.

What is left is the cube with a square hole 4 by 4 through it, from top to bottom.

**As a set:**

$$[0,10]^3 \setminus \bigl([3,7]\times[3,7]\times[0,10]\bigr)$$

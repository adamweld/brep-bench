+++
title = "A cube with a pocket cut flush into its base"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/A5"

[volume]
value = 920
exact = "1000 − 4·4·5"
tex = '10^3 - 4\cdot 4\cdot 5'

[area]
value = 680
exact = "6·10² − 4² + 4² + 4·4·5: the base loses the opening, the pocket adds its ceiling and four walls"
tex = '6\cdot 10^2 - 4^2 + 4^2 + 4\cdot 4\cdot 5'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (3, 3, 0) to (7, 7, 5): 4 by 4 and 5 high, standing on the xy plane inside the cube's footprint, its bottom in the plane of the cube's bottom. Cut the block away from the cube.

What is left is the cube with a square pocket 5 deep, open in its bottom face.

**As a set:**

$$[0,10]^3 \setminus \bigl([3,7]\times[3,7]\times[0,5]\bigr)$$

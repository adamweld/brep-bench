+++
title = "A block standing half out of a cube's top, less the cube"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/boptuc_simple/A7"

[volume]
value = 150
exact = "6·5·5: the block's part above the cube"
tex = '6\cdot 5\cdot 5'

[area]
value = 170
exact = "2·(6·5 + 6·5 + 5·5): the result is a block 6 by 5 by 5"
tex = '2\,(6\cdot 5 + 6\cdot 5 + 5\cdot 5)'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (0, 0, 5) to (6, 5, 15). The block begins halfway up the cube, lies inside it, and leaves it through the top face z = 10. Its faces x = 0 and y = 0 lie in the cube's faces. Cut the cube away from the block.

What is left is the block from (0, 0, 10) to (6, 5, 15), standing on the cube's top face.

**As a set:**

$$[0,6]\times[0,5]\times[5,15] \setminus [0,10]^3$$

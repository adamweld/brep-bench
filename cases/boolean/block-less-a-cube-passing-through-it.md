+++
title = "A bar through a cube, less the cube: two stubs"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "two-pieces"]
origin = "OCCT tests/boolean/boptuc_simple/F1"

[volume]
value = 240
exact = "2·(4·5·6): two stubs"
tex = '2\,(4\cdot 5\cdot 6)'

[area]
value = 296
exact = "2·2·(4·5 + 4·6 + 5·6) = 2·148"
tex = '2\cdot 2\,(4\cdot 5 + 4\cdot 6 + 5\cdot 6)'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a bar from (3, −5, 0) to (7, 15, 6). The bar lies along y, enters the cube through its face y = 0 and leaves through the opposite face y = 10; it stands on the cube's base plane z = 0 and is 6 high, inside the cube's height, and 4 wide, between x = 3 and x = 7, inside its width. Cut the cube away from the bar.

What is left is two separate blocks 4 by 5 by 6: one from (3, −5, 0) to (7, 0, 6) and one from (3, 10, 0) to (7, 15, 6).

**As a set:**

$$[3,7]\times[-5,15]\times[0,6] \setminus [0,10]^3$$

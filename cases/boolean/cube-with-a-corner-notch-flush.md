+++
title = "A cube with a corner notched flush with its base"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/B5"

[volume]
value = 875
exact = "1000 − 5·5·5"
tex = '10^3 - 5\cdot 5\cdot 5'

[area]
value = 600
exact = "6·10²: the notch takes three squares of 5² from the cube's faces and gives three of its own"
tex = '6\cdot 10^2 - 3\cdot 5^2 + 3\cdot 5^2'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (5, 5, 0) to (15, 15, 5), half the cube's height, standing on the xy plane with its bottom in the plane of the cube's bottom and overlapping the cube's footprint from 5 to 10 in x and y. Cut the block away from the cube.

What is left is the cube with a notch 5 by 5 by 5 taken out of the corner at (10, 10, 0).

**As a set:**

$$[0,10]^3 \setminus \bigl([5,15]\times[5,15]\times[0,5]\bigr)$$

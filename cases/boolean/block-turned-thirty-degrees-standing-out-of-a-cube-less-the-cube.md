+++
title = "A block turned 30°, its corners crossing a cube's sides, standing out through the top, less the cube"
tags = ["boolean", "cut", "analytic", "planar", "turned"]
origin = "OCCT tests/boolean/boptuc_simple/Q2"

[volume]
value = 387.97938089542851
exact = "V = 64·10 − 4·P, P = 160 − 56√3 the area where the block's section and the cube's plan overlap, over the 4 the block runs below the cube's top; 224√3"
tex = '640 - 4\,(160 - 56\sqrt3) = 224\sqrt3'

[area]
value = 360.57437415779593
exact = "area = 64 (block top) + P (the cube's top face where the block stood on it) + 4·(14√3 − 24) (block bottom, in four corner pieces) + 192 (the block's sides above the cube, 4·8·6) + 4·(8√3 − 8) (the corner pieces' side faces, over height 4) + 4·4·(8 − 4√3) (the cube's sides where the corners were), P = 160 − 56√3: 416 − 32√3"
tex = '416 - 32\sqrt3'
+++

A cube with x, y and z from 0 to 10.

A block whose section is a square of side 8: the square [−4, 4]² about the point (5, 5), turned 30° counterclockwise seen from above about the vertical axis through (5, 5). It runs from z = 6 to z = 16. Cut the cube away from the block.

The block's faces are 4 from the vertical axis through (5, 5), inside the cube's sides, which are 5 from it. Each of its four vertical edges, at distance 4√2 ≈ 5.66, pokes 2√3 − 3 ≈ 0.46 through one of the cube's side faces, at a slant. The block starts 4 below the cube's top face and stands 6 above it. What is left is the part of the block above z = 10, which is a whole block 6 tall, and under it, from z = 6 to z = 10, four small wedges, one at each corner of the block, where it pokes out of the cube. The wedges join the block above them: it is one piece.

**As a set:**

$$\bigl(Q\times[6,16]\bigr) \setminus [0,10]^3,\quad Q = \text{the square } [-4,4]^2 \text{ about } (5,5), \text{ turned } 30^\circ$$

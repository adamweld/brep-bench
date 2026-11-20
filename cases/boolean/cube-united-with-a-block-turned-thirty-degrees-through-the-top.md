+++
title = "A cube united with a block turned 30°, its corners crossing the cube's sides, standing out through the top"
tags = ["boolean", "union", "analytic", "planar", "turned"]
origin = "OCCT tests/boolean/bopfuse_simple/Q2"

[volume]
value = 1387.9793808954285
exact = "V = 1000 + 64·10 − 4·P, P = 160 − 56√3 the area where the block's section and the cube's overlap (the 64 less four corner triangles, each 14√3 − 24)"
tex = '1000 + 640 - 4\,(160 - 56\sqrt3)'

[area]
value = 800.26656797432648
exact = "area = 600 + 448 − 2P − 4·(8 − 4√3)·4 − (40 − 8√3)·4, P = 160 − 56√3: the cube and the block, less the cube's top and the block's bottom where they overlap (P each), the four chords of the cube's sides in the block (8 − 4√3 each) over 4 in height, and the part of the block's outline inside the cube (40 − 8√3) over 4 in height"
tex = '600 + 448 - 2P - 4\,(8 - 4\sqrt3)\cdot 4 - (40 - 8\sqrt3)\cdot 4,\quad P = 160 - 56\sqrt3'
+++

A cube with x, y and z from 0 to 10.

A block whose section is a square of side 8: the square [−4, 4]² about the point (5, 5), turned 30° counterclockwise seen from above about the vertical axis through (5, 5). It runs from z = 6 to z = 16. Make the union of the cube and the block.

The block's faces are 4 from the vertical axis through (5, 5), inside the cube's sides, which are 5 from it. Each of its four vertical edges, at distance 4√2 ≈ 5.66, pokes 2√3 − 3 ≈ 0.46 through one of the cube's side faces, at a slant. The block starts 4 below the cube's top face and stands 6 above it.

**As a set:**

$$\bigl([0,10]^3\bigr) \cup \bigl(Q\times[6,16]\bigr),\quad Q = \text{the square } [-4,4]^2 \text{ about } (5,5), \text{ turned } 30^\circ$$

+++
title = "A cube united with a block turned 30°, its corners crossing the cube's sides, in a band touching neither top nor bottom"
tags = ["boolean", "union", "analytic", "planar", "turned"]
origin = "OCCT tests/boolean/bopfuse_simple/Q7"

[volume]
value = 1003.9793808954285
exact = "V = 1000 + 64·4 − 4·P, P = 160 − 56√3 the area where the block's section and the cube's overlap"
tex = '1000 + 256 - 4\,(160 - 56\sqrt3)'

[area]
value = 608.26656797432648
exact = "area = 600 + (2·64 + 32·4) − 2P − 4·(8 − 4√3)·4 − (40 − 8√3)·4, P = 160 − 56√3: the cube and the block, less the block's top and bottom where inside the cube (P each), the chords of the cube's sides in the block (8 − 4√3 each, four of them) over 4 in height, and the part of the block's outline inside the cube (40 − 8√3) over 4 in height"
tex = '600 + 256 - 2P - 4\,(8 - 4\sqrt3)\cdot 4 - (40 - 8\sqrt3)\cdot 4,\quad P = 160 - 56\sqrt3'
+++

A cube with x, y and z from 0 to 10.

A block whose section is a square of side 8: the square [−4, 4]² about the point (5, 5), turned 30° counterclockwise seen from above about the vertical axis through (5, 5). It runs from z = 3 to z = 7, inside the cube's height. Make the union of the cube and the block.

Each of the block's four vertical edges, at distance 4√2 ≈ 5.66 from the axis, pokes 2√3 − 3 ≈ 0.46 out through one of the cube's side faces, at a slant. The block's top and bottom faces are inside the cube. The union is the cube with four small wedges added to its sides.

**As a set:**

$$\bigl([0,10]^3\bigr) \cup \bigl(Q\times[3,7]\bigr),\quad Q = \text{the square } [-4,4]^2 \text{ about } (5,5), \text{ turned } 30^\circ$$

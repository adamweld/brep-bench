+++
title = "A cube cut by a turned block that touches only its vertical edge"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "edge-contact"]
origin = "OCCT tests/boolean/bopcut_simple/M8"

[volume]
value = 1000.0
exact = "the cube whole: 10³; the cut removes no volume"
tex = '10^3'

[area]
value = 600.0
exact = "the cube whole: 6·10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block turned about the vertical, standing from z = 0 up to z = 5. The block's footprint is a rectangle 5 by 5, turned about the vertical: from its corner (10, 10) its edges run 5 along (4/5, 3/5) and 5 along (−3/5, 4/5). Its other footprint corners are (14, 13), (11, 17) and (7, 14). Cut the block away from the cube.

The block's corner (10, 10, 0) is the cube's corner, and the block's edges run away from the cube (the footprint lies in y ≥ 10): the block's vertical edge from (10, 10, 0) to (10, 10, 5) lies along the cube's vertical edge, and that segment is all the two solids share. The block's base is in the cube's base plane z = 0, and its top is at z = 5. The cut removes nothing, and what is left is the cube.

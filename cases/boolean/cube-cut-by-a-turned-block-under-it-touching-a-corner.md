+++
title = "A cube cut by a turned block under it that touches only a corner"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "vertex-contact"]
origin = "OCCT tests/boolean/bopcut_simple/N6"

[volume]
value = 1000.0
exact = "the cube whole: 10³; the cut removes no volume"
tex = '10^3'

[area]
value = 600.0
exact = "the cube whole: 6·10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block turned about the vertical, standing from z = -5 up to z = 0. The block's footprint is a rectangle 5 by 5, turned about the vertical: from its corner (10, 10) its edges run 5 along (4/5, 3/5) and 5 along (−3/5, 4/5). Its other footprint corners are (14, 13), (11, 17) and (7, 14). Cut the block away from the cube.

The block's corner (10, 10, 0) is the cube's corner (10, 10, 0), and the block's footprint lies in y ≥ 10, away from the cube's, with its top face in the plane z = 0. The two solids touch in that one point and share nothing else. The cut removes nothing, and what is left is the cube.

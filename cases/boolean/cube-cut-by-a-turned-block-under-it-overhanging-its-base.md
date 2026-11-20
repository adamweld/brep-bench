+++
title = "A cube cut by a turned block under it, overhanging its base"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "face-contact"]
origin = "OCCT tests/boolean/bopcut_simple/N4"

[volume]
value = 1000.0
exact = "the cube whole: 10³; the cut removes no volume"
tex = '10^3'

[area]
value = 600.0
exact = "the cube whole: 6·10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block turned about the vertical, standing from z = -5 up to z = 0. The block's footprint is a rectangle 5 by 5, turned about the vertical: from its corner (5, 7) its edges run 5 along (4/5, 3/5) and 5 along (−3/5, 4/5). Its other footprint corners are (9, 10), (6, 14) and (2, 11). Cut the block away from the cube.

One of the block's corners, (5, 7), is strictly inside the cube's footprint, another, (9, 10), is on the cube's face y = 10, and the other two are outside; no corner of the cube is inside the block's. The block stands below the cube's base plane z = 0, its top face in that plane, so the two solids touch over the part of that face inside the cube's base, the quadrilateral (5, 7), (9, 10), (11/4, 10), area 75/8, and share no volume. The cut removes nothing, and what is left is the cube.

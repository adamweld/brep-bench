+++
title = "A cube cut by a turned block under it, touching within its base"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "face-contact"]
origin = "OCCT tests/boolean/bopcut_simple/N3"

[volume]
value = 1000.0
exact = "the cube whole: 10³; the cut removes no volume"
tex = '10^3'

[area]
value = 600.0
exact = "the cube whole: 6·10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block turned about the vertical, standing from z = -5 up to z = 0. The block's footprint is a rectangle 5 by 5, turned about the vertical: from its corner (3, 1) its edges run 5 along (4/5, 3/5) and 5 along (−3/5, 4/5). Its other footprint corners are (7, 4), (4, 8) and (0, 5). Cut the block away from the cube.

The block's footprint lies within the cube's: three of its corners are strictly inside, and the fourth, (0, 5), is on the cube's face x = 0. The block stands below the cube's base plane z = 0, its top face in that plane, so the two solids touch over that face, a square of area 25 lying in the cube's base face, and share no volume. The cut removes nothing, and what is left is the cube.

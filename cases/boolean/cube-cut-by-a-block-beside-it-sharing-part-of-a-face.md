+++
title = "A cube cut by a raised block standing against its side"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "face-contact"]
origin = "OCCT tests/boolean/bopcut_simple/P6"

[volume]
value = 1000.0
exact = "the cube whole: 10³; the cut removes no volume"
tex = '10^3'

[area]
value = 600.0
exact = "the cube whole: 6·10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (10, 3, 5) to (15, 8, 15), standing against the cube's face x = 10, from the cube's mid-height up to 5 above its top. The block's face x = 10 touches the cube's face x = 10 over the rectangle 3 ≤ y ≤ 8, 5 ≤ z ≤ 10, part of each. Cut the block away from the cube. The two share no volume; the cut removes nothing, and what is left is the cube.

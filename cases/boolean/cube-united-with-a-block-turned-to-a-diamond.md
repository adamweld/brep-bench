+++
title = "A cube united with a block turned to a diamond, standing out through the top"
tags = ["boolean", "union", "analytic", "planar", "turned", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/P5"

[volume]
value = 2750.0
exact = "V = 1000 + 200·10 − 50·5 (the block's section is 200; its overlap with the cube is a triangle of 50 over a height of 5)"
tex = '1000 + 200\cdot 10 - 50\cdot 5'

[area]
value = 1294.9747468305833
exact = "area = (600 − 50 − 100) + (200 + 150 + 350√2): the cube less its top where the block covers it (50) and its faces x = 10 and y = 10 from z = 5 to 10 (50 each); the block's top (200), its bottom outside the cube (200 − 50), and its four sides (each 10√2 × 10) less the strip of the one through the cube, z from 5 to 10"
tex = '450 + 150 + 200 + 350\sqrt2'
+++

A cube with x, y and z from 0 to 10.

A block whose section is a square of side 10√2, turned 45° about the vertical so that it is a diamond: its vertical edges are at (0, 10), (10, 0), (20, 10) and (10, 20), and it runs from z = 5 to z = 15. Make the union of the cube and the block.

Two of the block's vertical edges, at (0, 10) and (10, 0), lie on the cube's vertical edges, from z = 5 to z = 10, and the block's face between them runs through the cube along the plane x + y = 10 for those heights. The block's centre line, through (10, 10), is the cube's fourth vertical edge, so the block covers the half of the cube on that side of the plane, from z = 5 up. The block's top, z = 15, is 5 above the cube's top face.

**As a set:**

$$\bigl([0,10]^3\bigr) \cup \bigl(\{\,|x-10| + |y-10| \le 10\,\}\times[5,15]\bigr)$$
